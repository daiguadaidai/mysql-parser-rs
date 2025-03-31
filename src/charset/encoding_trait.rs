use crate::charset::encoding::EncodingTp;
use crate::common::error::CustomError;
use crate::mysql::errcode;
use crate::mysql::errname::mysql_err_name;
use formatx::formatx;

pub trait EncodingTrait {
    fn name(&self) -> String;

    fn tp(&self) -> EncodingTp;

    fn peek(&self, src: &[u8]) -> Vec<u8>;

    fn mb_len(&self, s: &str) -> isize;

    fn is_valid(&self, src: &[u8]) -> bool;

    fn foreach<F>(&self, src: &[u8], op: i16, f: F)
    where
        // Fn(from: &[u8], to: &[u8], ok: bool) -> bool
        F: FnMut(&[u8], &[u8], bool) -> bool;

    fn transform(&self, dest: &mut Vec<u8>, src: &[u8], op: i16) -> Result<Vec<u8>, CustomError>;

    fn to_upper(&self, src: &str) -> String {
        src.to_uppercase()
    }

    fn to_lower(&self, src: &str) -> String {
        src.to_lowercase()
    }
}

const REPLACEMENT_BYTES: &[u8] = &[0xEF, 0xBF, 0xBD];

// beginWithReplacementChar check if dst has the prefix '0xEFBFBD'.
pub fn begin_with_replacement_char(dst: &[u8]) -> bool {
    dst.starts_with(REPLACEMENT_BYTES)
}

pub fn generate_encoding_err(name: &str, invalid_bytes: &[u8]) -> Result<(), CustomError> {
    let msg = mysql_err_name
        .get(&errcode::ERR_INVALID_CHARACTER_STRING)
        .unwrap();
    let arg: String = invalid_bytes
        .iter()
        .map(|b| format!("{:02X}", b)) // 将每个字节转换为两位十六进制
        .collect(); // 拼接成字符串
    let err_str = formatx!(&msg.raw, name, arg)?;

    Err(CustomError::Normal(err_str))
}
