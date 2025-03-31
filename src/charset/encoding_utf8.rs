use crate::charset::charset;
use crate::charset::encoding::{
    EncodingTp, EncodingTrait, OP_COLLECR_TO, OP_COLLECT_FROM, OP_SKIP_ERROR, OP_TRUNCATE_REPLACE,
    OP_TRUNCATE_TRIM,
};
use crate::charset::encoding_trait::generate_encoding_err;
use crate::common::error::CustomError;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref encoding_utf8_impl: EncodingUTF8 = EncodingUTF8 {
        enc: encoding_rs::UTF_8,
    };
}

pub struct EncodingUTF8 {
    pub enc: &'static encoding_rs::Encoding,
}

impl EncodingTrait for EncodingUTF8 {
    fn name(&self) -> String {
        charset::CHARSET_UTF8MB4.to_string()
    }

    fn tp(&self) -> EncodingTp {
        EncodingTp::UTF8
    }

    fn peek(&self, src: &[u8]) -> Vec<u8> {
        let mut next_len = 4;
        if src.len() == 0 || src[0] < 0x80 {
            next_len = 1
        } else if src[0] < 0xe0 {
            next_len = 2;
        } else if src[0] < 0xf0 {
            next_len = 3;
        }

        if src.len() < next_len {
            src.to_vec()
        } else {
            src[..next_len].to_vec()
        }
    }

    fn mb_len(&self, s: &str) -> isize {
        let size = s.chars().count();
        if size <= 1 {
            0
        } else {
            size as isize
        }
    }

    fn is_valid(&self, src: &[u8]) -> bool {
        std::str::from_utf8(src).is_ok()
    }

    fn foreach<F>(&self, src: &[u8], op: i16, f: F)
    where
        F: FnMut(&[u8], &[u8], bool) -> bool,
    {
        let mut i = 0;
        while i < src.len() {
            // 解码 UTF-8 字符
            let (rv, w) = match std::str::from_utf8(&src[i..]) {
                Ok(s) => {
                    let rv = s.chars().next().unwrap(); // 获取第一个字符
                    (rv, rv.len_utf8()) // 获取字符的字节长度
                }
                Err(e) => {
                    // 解码失败，返回 RuneError 和无效字节长度
                    ('\u{FFFD}', e.valid_up_to().max(1))
                }
            };

            // 检查是否遇到错误
            let meet_err = rv == '\u{FFFD}' && w == 1;

            // 调用回调函数
            if !f(&src[i..i + w], &src[i..i + w], !meet_err) {
                return;
            }

            // 移动到下一个字符
            i += w;
        }
    }

    fn transform(&self, dest: &mut Vec<u8>, src: &[u8], op: i16) -> Result<Vec<u8>, CustomError> {
        if self.is_valid(src) {
            return Ok(Vec::from(src));
        }

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
            } else if op & OP_COLLECR_TO {
                dest.extend_from_slice(to);
            }

            return true;
        };

        self.foreach(&src, op, callback);

        Ok(dest.to_vec())
    }
}
