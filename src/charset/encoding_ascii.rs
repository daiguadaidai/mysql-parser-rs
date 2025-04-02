use crate::charset::charset;
use crate::charset::encoding::{
    EncodingTp, OP_COLLECR_TO, OP_COLLECT_FROM, OP_SKIP_ERROR, OP_TRUNCATE_REPLACE,
    OP_TRUNCATE_TRIM,
};
use crate::charset::encoding_lazy_static::encoding_utf8_impl;
use crate::charset::encoding_trait::{generate_encoding_err, EncodingTrait};
use crate::common::error::CustomError;
use logos::Source;

#[allow(dead_code)]
pub struct EncodingASCII {
    pub enc: &'static encoding_rs::Encoding,
}

impl EncodingTrait for EncodingASCII {
    fn name(&self) -> String {
        charset::CHARSET_ASCII.to_string()
    }

    fn tp(&self) -> EncodingTp {
        EncodingTp::ASCII
    }

    fn peek(&self, src: &[u8]) -> Vec<u8> {
        if src.len() == 0 {
            return src.to_vec();
        }

        src[..1].to_vec()
    }

    fn mb_len(&self, s: &str) -> isize {
        let size = s.len();
        if size <= 1 {
            0
        } else {
            size as isize
        }
    }

    fn is_valid(&self, src: &[u8]) -> bool {
        src.iter().all(|&b| b <= 127)
    }

    fn foreach<F>(&self, src: &[u8], _: i16, f: F)
    where
        F: FnMut(&[u8], &[u8], bool) -> bool,
    {
        let mut i = 0;
        while i < src.len() {
            let (w, ok) = if src[i] > 127 {
                // 如果不是 ASCII 字符，获取 UTF-8 字符的宽度
                (encoding_utf8_impl.peek(&src[i..]).len(), false)
            } else {
                // 如果是 ASCII 字符，宽度为 1
                (1, true)
            };

            // 调用回调函数
            if !f(&src[i..i + w], &src[i..i + w], ok) {
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
            } else if op & OP_COLLECR_TO != 0 {
                dest.extend_from_slice(to);
            }

            return true;
        };

        self.foreach(&src, op, callback);

        Ok(dest.to_vec())
    }
}
