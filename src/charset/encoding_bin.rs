use crate::charset::charset;
use crate::charset::encoding::EncodingTp;
use crate::charset::encoding_trait::EncodingTrait;
use crate::common::error::CustomError;

#[allow(dead_code)]
pub struct EncodingBin {
    pub enc: &'static encoding_rs::Encoding,
}

impl EncodingTrait for EncodingBin {
    fn name(&self) -> String {
        charset::CHARSET_BIN.to_string()
    }

    fn tp(&self) -> EncodingTp {
        EncodingTp::Bin
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

    fn is_valid(&self, _: &[u8]) -> bool {
        true
    }

    fn foreach<F>(&self, src: &[u8], _: i16, f: F)
    where
        F: FnMut(&[u8], &[u8], bool) -> bool,
    {
        let mut i = 0;
        while i < src.len() {
            // 调用回调函数
            if !f(&src[i..i + 1], &src[i..i + 1], true) {
                return;
            }

            // 移动到下一个字符
            i += 1;
        }
    }

    fn transform(&self, _: &mut Vec<u8>, src: &[u8], _: i16) -> Result<Vec<u8>, CustomError> {
        Ok(src.to_vec())
    }
}
