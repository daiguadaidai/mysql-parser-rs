use crate::charset::charset;
use crate::charset::encoding::EncodingTp;
use crate::charset::encoding_trait::EncodingTrait;
use crate::common::error::CustomError;

#[allow(dead_code)]
pub struct EncodingLatin1 {
    pub enc: &'static encoding_rs::Encoding,
}

impl EncodingTrait for EncodingLatin1 {
    fn name(&self) -> String {
        charset::CHARSET_LATIN1.to_string()
    }

    fn tp(&self) -> EncodingTp {
        EncodingTp::Latin1
    }

    fn peek(&self, src: &[u8]) -> Vec<u8> {
        if src.len() == 0 {
            return src.to_vec();
        }

        src[..1].to_vec()
    }

    fn mb_len(&self, s: &str) -> isize {
        let size = s.chars().count();
        if size <= 1 {
            0
        } else {
            size as isize
        }
    }

    fn is_valid(&self, _: &[u8]) -> bool {
        true
    }

    fn foreach<F>(&self, src: &[u8], _: i16, f: F)
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

    fn transform(&self, _: &mut Vec<u8>, src: &[u8], _: i16) -> Result<Vec<u8>, CustomError> {
        Ok(src.to_vec())
    }
}
