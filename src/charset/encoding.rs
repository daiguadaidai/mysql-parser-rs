use crate::charset::encoding_ascii::EncodingASCII;
use crate::charset::encoding_bin::EncodingBin;
use crate::charset::encoding_gbk::EncodingGBK;
use crate::charset::encoding_latin1::EncodingLatin1;
use crate::charset::encoding_trait::EncodingTrait;
use crate::charset::encoding_utf8::EncodingUTF8;
use crate::charset::encoding_utf8mb3_strict::EncodingUtf8Mb3Strict;
use crate::common::error::CustomError;

// Encoding provide encode/decode functions for a string with a specific charset.
pub enum Encoding {
    UTF8(EncodingUTF8),
    Utf8mb3Strict(EncodingUtf8Mb3Strict),
    GBK(EncodingGBK),
    Latin1(EncodingLatin1),
    Bin(EncodingBin),
    ASCII(EncodingASCII),
}

impl EncodingTrait for Encoding {
    // Name is the name of the encoding.
    fn name(&self) -> String {
        match self {
            Encoding::UTF8(enc) => enc.name(),
            Encoding::Utf8mb3Strict(enc) => enc.name(),
            Encoding::GBK(enc) => enc.name(),
            Encoding::Latin1(enc) => enc.name(),
            Encoding::Bin(enc) => enc.name(),
            Encoding::ASCII(enc) => enc.name(),
        }
    }

    // Tp is the type of the encoding.
    fn tp(&self) -> EncodingTp {
        match self {
            Encoding::UTF8(enc) => enc.tp(),
            Encoding::Utf8mb3Strict(enc) => enc.tp(),
            Encoding::GBK(enc) => enc.tp(),
            Encoding::Latin1(enc) => enc.tp(),
            Encoding::Bin(enc) => enc.tp(),
            Encoding::ASCII(enc) => enc.tp(),
        }
    }

    // Peek returns the next char.
    fn peek(&self, src: &[u8]) -> Vec<u8> {
        match self {
            Encoding::UTF8(enc) => enc.peek(src),
            Encoding::Utf8mb3Strict(enc) => enc.peek(src),
            Encoding::GBK(enc) => enc.peek(src),
            Encoding::Latin1(enc) => enc.peek(src),
            Encoding::Bin(enc) => enc.peek(src),
            Encoding::ASCII(enc) => enc.peek(src),
        }
    }

    // MbLen returns multiple byte length, if the next character is single byte, return 0.
    fn mb_len(&self, s: &str) -> isize {
        match self {
            Encoding::UTF8(enc) => enc.mb_len(s),
            Encoding::Utf8mb3Strict(enc) => enc.mb_len(s),
            Encoding::GBK(enc) => enc.mb_len(s),
            Encoding::Latin1(enc) => enc.mb_len(s),
            Encoding::Bin(enc) => enc.mb_len(s),
            Encoding::ASCII(enc) => enc.mb_len(s),
        }
    }

    // IsValid checks whether the utf-8 bytes can be convert to valid string in current encoding.
    fn is_valid(&self, src: &[u8]) -> bool {
        match self {
            Encoding::UTF8(enc) => enc.is_valid(src),
            Encoding::Utf8mb3Strict(enc) => enc.is_valid(src),
            Encoding::GBK(enc) => enc.is_valid(src),
            Encoding::Latin1(enc) => enc.is_valid(src),
            Encoding::Bin(enc) => enc.is_valid(src),
            Encoding::ASCII(enc) => enc.is_valid(src),
        }
    }

    // Foreach iterates the characters in current encoding.
    fn foreach<F>(&self, src: &[u8], op: i16, mut f: F)
    where
        // Fn(from: &[u8], to: &[u8], ok: bool) -> bool
        F: FnMut(&[u8], &[u8], bool) -> bool,
    {
        match self {
            Encoding::UTF8(enc) => enc.foreach(src, op, &mut f),
            Encoding::Utf8mb3Strict(enc) => enc.foreach(src, op, &mut f),
            Encoding::GBK(enc) => enc.foreach(src, op, &mut f),
            Encoding::Latin1(enc) => enc.foreach(src, op, &mut f),
            Encoding::Bin(enc) => enc.foreach(src, op, &mut f),
            Encoding::ASCII(enc) => enc.foreach(src, op, &mut f),
        }
    }

    // Transform map the bytes in src to dest according to Op.
    // **the caller should initialize the dest if it wants to avoid memory alloc every time,
    //   or else it will always make a new one**
    //
    // **the returned array may be the alias of `src`, edit the returned array on your own risk**
    fn transform(&self, dest: &mut Vec<u8>, src: &[u8], op: i16) -> Result<Vec<u8>, CustomError> {
        match self {
            Encoding::UTF8(enc) => enc.transform(dest, src, op),
            Encoding::Utf8mb3Strict(enc) => enc.transform(dest, src, op),
            Encoding::GBK(enc) => enc.transform(dest, src, op),
            Encoding::Latin1(enc) => enc.transform(dest, src, op),
            Encoding::Bin(enc) => enc.transform(dest, src, op),
            Encoding::ASCII(enc) => enc.transform(dest, src, op),
        }
    }

    // ToUpper change a string to uppercase.
    fn to_upper(&self, src: &str) -> String {
        match self {
            Encoding::UTF8(enc) => enc.to_upper(src),
            Encoding::Utf8mb3Strict(enc) => enc.to_upper(src),
            Encoding::GBK(enc) => enc.to_upper(src),
            Encoding::Latin1(enc) => enc.to_upper(src),
            Encoding::Bin(enc) => enc.to_upper(src),
            Encoding::ASCII(enc) => enc.to_upper(src),
        }
    }

    // ToLower change a string to lowercase.
    fn to_lower(&self, src: &str) -> String {
        match self {
            Encoding::UTF8(enc) => enc.to_lower(src),
            Encoding::Utf8mb3Strict(enc) => enc.to_lower(src),
            Encoding::GBK(enc) => enc.to_lower(src),
            Encoding::Latin1(enc) => enc.to_lower(src),
            Encoding::Bin(enc) => enc.to_lower(src),
            Encoding::ASCII(enc) => enc.to_lower(src),
        }
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
//revive:disable
pub const OP_REPLACE_NO_ERR: i16 =
    OP_FROM_UTF8 | OP_TRUNCATE_REPLACE | OP_COLLECT_FROM | OP_SKIP_ERROR;
#[allow(dead_code)]
pub const OP_REPLACE: i16 = OP_FROM_UTF8 | OP_TRUNCATE_REPLACE | OP_COLLECT_FROM;
pub const OP_ENCODE: i16 = OP_FROM_UTF8 | OP_TRUNCATE_TRIM | OP_COLLECR_TO;
#[allow(dead_code)]
pub const OP_ENCODE_NO_ERR: i16 = OP_ENCODE | OP_SKIP_ERROR;
#[allow(dead_code)]
pub const OP_ENCODE_REPLACE: i16 = OP_FROM_UTF8 | OP_TRUNCATE_REPLACE | OP_COLLECR_TO;
pub const OP_DECODE: i16 = OP_TO_UTF8 | OP_TRUNCATE_TRIM | OP_COLLECR_TO;
#[allow(dead_code)]
pub const OP_DECODE_NO_ERR: i16 = OP_DECODE | OP_SKIP_ERROR;
#[allow(dead_code)]
pub const OP_DECODE_REPLACE: i16 = OP_TO_UTF8 | OP_TRUNCATE_REPLACE | OP_COLLECR_TO;
