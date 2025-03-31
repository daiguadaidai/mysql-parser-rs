use crate::charset::charset::CHARSET_UTF8MB4;
use crate::charset::encoding_trait::EncodingTrait;
use crate::common::error::CustomError;
use lazy_static::lazy_static;
use std::collections::HashMap;

// Encoding provide encode/decode functions for a string with a specific charset.
pub enum Encoding {
    None,
}

impl EncodingTrait for Encoding {
    // Name is the name of the encoding.
    fn name(&self) -> String {
        match self {
            Encoding::None => "".to_string(),
        }
    }

    // Tp is the type of the encoding.
    fn tp(&self) -> EncodingTp {
        match self {
            Encoding::None => EncodingTp::None,
        }
    }

    // Peek returns the next char.
    fn peek(&self, src: &[u8]) -> Vec<u8> {
        match self {
            Encoding::None => src.to_vec(),
        }
    }

    // MbLen returns multiple byte length, if the next character is single byte, return 0.
    fn mb_len(&self, s: &str) -> isize {
        match self {
            Encoding::None => s.len() as isize,
        }
    }

    // IsValid checks whether the utf-8 bytes can be convert to valid string in current encoding.
    fn is_valid(&self, src: &[u8]) -> bool {
        match self {
            Encoding::None => false,
        }
    }

    // Foreach iterates the characters in current encoding.
    fn foreach<F>(&self, src: &[u8], op: i16, mut f: F)
    where
        // Fn(from: &[u8], to: &[u8], ok: bool) -> bool
        F: FnMut(&[u8], &[u8], bool) -> bool,
    {
    }

    // Transform map the bytes in src to dest according to Op.
    // **the caller should initialize the dest if it wants to avoid memory alloc every time,
    //   or else it will always make a new one**
    //
    // **the returned array may be the alias of `src`, edit the returned array on your own risk**
    fn transform(&self, dest: &mut Vec<u8>, src: &[u8], op: i16) -> Result<Vec<u8>, CustomError> {
        match self {
            Encoding::None => Ok(vec![]),
        }
    }

    // ToUpper change a string to uppercase.
    fn to_upper(&self, src: &str) -> String {
        match self {
            Encoding::None => "".to_string(),
        }
    }

    // ToLower change a string to lowercase.
    fn to_lower(&self, src: &str) -> String {
        match self {
            Encoding::None => "".to_string(),
        }
    }
}

pub enum EncodingTp {
    None,
    UTF8,
    UTF8MB3Strict,
    ASCII,
    Latin1,
    Bin,
    GBK,
}

// Op is used by Encoding.Transform.
pub const OP_FROM_UTF8: i16 = 1;
pub const OP_TO_UTF8: i16 = OP_FROM_UTF8 << 1;
pub const OP_TRUNCATE_TRIM: i16 = OP_FROM_UTF8 << 2;
pub const OP_TRUNCATE_REPLACE: i16 = OP_FROM_UTF8 << 3;
pub const OP_COLLECT_FROM: i16 = OP_FROM_UTF8 << 4;
pub const OP_COLLECR_TO: i16 = OP_FROM_UTF8 << 5;
pub const OP_SKIP_ERROR: i16 = OP_FROM_UTF8 << 6;

//revive:disable
pub const OP_REPLACE_NO_ERR: i16 =
    OP_FROM_UTF8 | OP_TRUNCATE_REPLACE | OP_COLLECT_FROM | OP_SKIP_ERROR;
pub const OP_REPLACE: i16 = OP_FROM_UTF8 | OP_TRUNCATE_REPLACE | OP_COLLECT_FROM;
pub const OP_ENCODE: i16 = OP_FROM_UTF8 | OP_TRUNCATE_TRIM | OP_COLLECR_TO;
pub const OP_ENCODE_NO_ERR: i16 = OP_ENCODE | OP_SKIP_ERROR;
pub const OP_ENCODE_REPLACE: i16 = OP_FROM_UTF8 | OP_TRUNCATE_REPLACE | OP_COLLECR_TO;
pub const OP_DECODE: i16 = OP_FROM_UTF8 | OP_TRUNCATE_TRIM | OP_COLLECR_TO;
pub const OP_DECODE_NO_ERR: i16 = OP_DECODE | OP_SKIP_ERROR;
pub const OP_DECODE_REPLACE: i16 = OP_FROM_UTF8 | OP_TRUNCATE_REPLACE | OP_COLLECR_TO;

lazy_static! {
    pub static ref encoding_map: HashMap<String, Encoding> = {
        let mut m = HashMap::<String, Encoding>::new();
        m.insert(CHARSET_UTF8MB4.to_string(), )

        m
    };
}
