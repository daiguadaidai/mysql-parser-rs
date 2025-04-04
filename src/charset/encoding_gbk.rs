use crate::charset::charset;
use crate::charset::encoding::{
    EncodingTp, OP_COLLECR_TO, OP_COLLECT_FROM, OP_FROM_UTF8, OP_SKIP_ERROR, OP_TO_UTF8,
    OP_TRUNCATE_REPLACE, OP_TRUNCATE_TRIM,
};
use crate::charset::encoding_lazy_static::encoding_utf8_impl;
use crate::charset::encoding_trait::{
    begin_with_replacement_char, generate_encoding_err, EncodingTrait,
};
use crate::common::error::CustomError;
use logos::Source;
use std::char::REPLACEMENT_CHARACTER;

pub struct EncodingGBK {
    pub enc: &'static encoding_rs::Encoding,
}

impl EncodingTrait for EncodingGBK {
    fn name(&self) -> String {
        charset::CHARSET_GBK.to_string()
    }

    fn tp(&self) -> EncodingTp {
        EncodingTp::GBK
    }

    fn peek(&self, src: &[u8]) -> Vec<u8> {
        let mut char_len = 2;
        if src.len() == 0 || src[0] < 0x80 {
            char_len = 1
        }
        if char_len < src.len() {
            return src[..char_len].to_vec();
        }

        src.to_vec()
    }

    fn mb_len(&self, s: &str) -> isize {
        let bs = s.as_bytes();

        if bs.len() < 2 {
            return 0;
        }

        if 0x81 <= bs[0] && bs[0] <= 0xfe {
            if (0x40 <= bs[1] && bs[1] <= 0x7e) || (0x80 <= bs[1] && bs[1] <= 0xfe) {
                return 2;
            }
        }

        0
    }

    fn is_valid(&self, src: &[u8]) -> bool {
        std::str::from_utf8(src).is_ok()
    }

    fn foreach<F>(&self, src: &[u8], op: i16, f: F)
    where
        F: FnMut(&[u8], &[u8], bool) -> bool,
    {
        let mut buf: [u8; 4] = [0; 4];
        let mut i = 0;
        while i < src.len() {
            let (data, w) = if op & OP_FROM_UTF8 != 0 {
                let w = encoding_utf8_impl.peek(&src[i..]).len();
                (self.gbk_encode(&mut buf, &src[i..i + w], false), w)
            } else {
                let w = self.peek(&src[i..]).len();
                (self.gbk_decode(&mut buf, &src[i..i + w], false), w)
            };
            let (meet_err, n_dst) = match data {
                Ok((n, _)) => (
                    op & OP_TO_UTF8 != 0 && begin_with_replacement_char(&buf[..n]),
                    n,
                ),
                Err(_) => (true, 0),
            };

            if !f(&src[i..i + w], &buf[..n_dst], !meet_err) {
                return;
            }

            i += w
        }
    }

    fn transform(&self, dest: &mut Vec<u8>, src: &[u8], op: i16) -> Result<Vec<u8>, CustomError> {
        if dest.is_empty() {
            dest.reserve(src.len());
        }
        dest.clear();

        let mut err: Result<(), CustomError> = Ok(());
        let callback = |from: &[u8], to: &[u8], ok: bool| -> bool {
            if !ok {
                if err.is_ok() && (op & OP_SKIP_ERROR == 0) {
                    err = generate_encoding_err(&self.name(), from)
                }
                if op & OP_TRUNCATE_TRIM != 0 {
                    return false;
                }
                if op & OP_TRUNCATE_REPLACE != 0 {
                    dest.push('?' as u8);
                    return true;
                }
            }

            if op & OP_COLLECT_FROM != 0 {
                dest.extend_from_slice(from);
            } else if op & OP_COLLECR_TO != 0 {
                dest.extend_from_slice(to);
            }

            return true;
        };

        self.foreach(&src, op, callback);

        let _ = err?;

        Ok(dest.to_vec())
    }

    fn to_upper(&self, src: &str) -> String {
        to_upper_special(&GBK_CASE, src)
    }

    fn to_lower(&self, src: &str) -> String {
        to_lower_special(&GBK_CASE, src)
    }
}

impl EncodingGBK {
    pub fn gbk_decode(
        &self,
        dest: &mut [u8],
        src: &[u8],
        at_eof: bool,
    ) -> Result<(usize, usize), CustomError> {
        if src.len() == 0 {
            return Ok((0, 0));
        }

        // 处理特定情况：如果第一个字节是 0x80，返回替换字符
        if src[0] == 0x80 {
            let encoded = REPLACEMENT_CHARACTER.encode_utf8(dest);
            return Ok((encoded.len(), 1));
        }

        // 否则，使用 GBK 解码器进行转换
        let (decoded_string, _encoding_used, had_errors) = self.enc.decode(src);
        // 如果解码失败且 at_eof = true，返回错误
        if had_errors && at_eof {
            return Err(CustomError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid GBK sequence at EOF",
            )));
        }

        let decoded_bytes = decoded_string.as_bytes();

        // 检查目标缓冲区是否足够大
        if dest.len() < decoded_bytes.len() {
            return Err(CustomError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Destination buffer too small",
            )));
        }

        // 将解码后的字节复制到目标缓冲区
        dest[..decoded_bytes.len()].copy_from_slice(decoded_bytes);

        // 返回写入的字节数、读取的字节数和错误
        Ok((decoded_bytes.len(), src.len()))
    }

    /// 转换方法，类似于 Go 的 Transform
    pub fn gbk_encode(
        &self,
        dst: &mut [u8],
        src: &[u8],
        at_eof: bool,
    ) -> Result<(usize, usize), CustomError> {
        // 检查是否包含 '€' 字符的 UTF-8 编码
        if src.starts_with(&[0xE2, 0x82, 0xAC]) {
            return Err(CustomError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid character: '€'",
            )));
        }

        // 将 src 转换为 &str（UTF-8）
        let src_str = match std::str::from_utf8(src) {
            Ok(s) => s,
            Err(e) => {
                // 如果 at_eof = true 且 src 不是有效的 UTF-8，返回错误
                if at_eof {
                    return Err(CustomError::IoError(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid UTF-8 sequence: {}", e),
                    )));
                }
                // 如果 at_eof = false，等待更多数据
                return Ok((0, 0));
            }
        };

        // 使用 GBK 编码器进行转换
        let (encoded_bytes, _encoding_used, had_errors) = self.enc.encode(src_str);

        // 如果编码失败且 at_eof = true，返回错误
        if had_errors && at_eof {
            return Err(CustomError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid UTF-8 sequence at EOF",
            )));
        }

        // 检查目标缓冲区是否足够大
        if dst.len() < encoded_bytes.len() {
            return Err(CustomError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Destination buffer too small",
            )));
        }

        // 将编码后的字节复制到目标缓冲区
        dst[..encoded_bytes.len()].copy_from_slice(&encoded_bytes);

        // 返回写入的字节数、读取的字节数和错误
        Ok((encoded_bytes.len(), src.len()))
    }
}

#[derive(Debug, Clone, Copy)]
struct CaseRange {
    lo: u32,         // 范围下限
    hi: u32,         // 范围上限
    delta: [i32; 3], // 大小写转换的 Delta 值
}

// 定义 GBKCase
const GBK_CASE: [CaseRange; 23] = [
    CaseRange {
        lo: 0x00E0,
        hi: 0x00E1,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x00E8,
        hi: 0x00EA,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x00EC,
        hi: 0x00ED,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x00F2,
        hi: 0x00F3,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x00F9,
        hi: 0x00FA,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x00FC,
        hi: 0x00FC,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x0101,
        hi: 0x0101,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x0113,
        hi: 0x0113,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x011B,
        hi: 0x011B,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x012B,
        hi: 0x012B,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x0144,
        hi: 0x0144,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x0148,
        hi: 0x0148,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x014D,
        hi: 0x014D,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x016B,
        hi: 0x016B,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x01CE,
        hi: 0x01CE,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x01D0,
        hi: 0x01D0,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x01D2,
        hi: 0x01D2,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x01D4,
        hi: 0x01D4,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x01D6,
        hi: 0x01D6,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x01D8,
        hi: 0x01D8,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x01DA,
        hi: 0x01DA,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x01DC,
        hi: 0x01DC,
        delta: [0, 0, 0],
    },
    CaseRange {
        lo: 0x216A,
        hi: 0x216B,
        delta: [0, 0, 0],
    },
];

// 自定义小写转换函数
fn to_lower_special(special_case: &[CaseRange], input: &str) -> String {
    input
        .chars()
        .map(|c| {
            let code = c as u32;
            // 检查字符是否在特殊规则范围内
            for range in special_case {
                if code >= range.lo && code <= range.hi {
                    // 根据 delta 规则进行转换
                    return char::from_u32(code.wrapping_add(range.delta[1] as u32)).unwrap_or(c);
                }
            }
            // 如果不在特殊规则范围内，默认转换为小写
            c.to_lowercase().next().unwrap_or(c)
        })
        .collect()
}

// 自定义大小写转换函数
fn to_upper_special(special_case: &[CaseRange], input: &str) -> String {
    input
        .chars()
        .map(|c| {
            let code = c as u32;
            // 检查字符是否在特殊规则范围内
            for range in special_case {
                if code >= range.lo && code <= range.hi {
                    // 根据 delta 规则进行转换
                    return char::from_u32(code.wrapping_add(range.delta[0] as u32)).unwrap_or(c);
                }
            }
            // 如果不在特殊规则范围内，默认转换为大写
            c.to_uppercase().next().unwrap_or(c)
        })
        .collect()
}
