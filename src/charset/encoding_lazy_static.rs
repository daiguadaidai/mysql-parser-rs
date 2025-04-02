use crate::charset::charset::{
    CHARSET_ASCII, CHARSET_GBK, CHARSET_LATIN1, CHARSET_UTF8, CHARSET_UTF8MB3, CHARSET_UTF8MB4,
};
use crate::charset::encoding::Encoding;
use crate::charset::encoding_ascii::EncodingASCII;
use crate::charset::encoding_bin::EncodingBin;
use crate::charset::encoding_gbk::EncodingGBK;
use crate::charset::encoding_latin1::EncodingLatin1;
use crate::charset::encoding_utf8::EncodingUTF8;
use crate::charset::encoding_utf8mb3_strict::EncodingUtf8Mb3Strict;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref encoding_utf8_impl: Encoding = Encoding::UTF8(EncodingUTF8 {
        enc: encoding_rs::UTF_8,
    });
    pub static ref encoding_utf8mb3_strict_impl: Encoding =
        Encoding::Utf8mb3Strict(EncodingUtf8Mb3Strict {
            enc: encoding_rs::UTF_8,
        });
    pub static ref encoding_gbk_impl: Encoding = Encoding::GBK(EncodingGBK {
        enc: encoding_rs::GBK,
    });
    pub static ref encoding_latin1_impl: Encoding = Encoding::Latin1(EncodingLatin1 {
        enc: encoding_rs::UTF_8,
    });
    pub static ref encoding_ascii_impl: Encoding = Encoding::ASCII(EncodingASCII {
        enc: encoding_rs::UTF_8,
    });
    pub static ref encoding_bin_impl: Encoding = Encoding::Bin(EncodingBin {
        enc: encoding_rs::UTF_8,
    });
    pub static ref encoding_map: HashMap<&'static str, &'static Encoding> = {
        let mut m = HashMap::<&'static str, &'static Encoding>::new();
        m.insert(CHARSET_UTF8MB4, &encoding_utf8_impl);
        m.insert(CHARSET_UTF8, &encoding_utf8_impl);
        m.insert(CHARSET_UTF8MB3, &encoding_utf8mb3_strict_impl);
        m.insert(CHARSET_GBK, &encoding_gbk_impl);
        m.insert(CHARSET_LATIN1, &encoding_latin1_impl);
        m.insert(CHARSET_ASCII, &encoding_ascii_impl);

        m
    };
}
