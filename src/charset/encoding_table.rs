use lazy_static::lazy_static;
use std::collections::HashMap;

const NAME_UTF8: &str = "utf-8";
const NAME_BINARY: &str = "binary";
const NAME_IBM866: &str = "ibm866";
const NAME_ISO_8859_2: &str = "iso-8859-2";
const NAME_ISO_8859_3: &str = "iso-8859-3";
const NAME_ISO_8859_4: &str = "iso-8859-4";
const NAME_ISO_8859_5: &str = "iso-8859-5";
const NAME_ISO_8859_6: &str = "iso-8859-6";
const NAME_ISO_8859_7: &str = "iso-8859-7";
const NAME_ISO_8859_8: &str = "iso-8859-8";
const NAME_ISO_8859_8_I: &str = "iso-8859-8-i";
const NAME_ISO_8859_10: &str = "iso-8859-10";
const NAME_ISO_8859_13: &str = "iso-8859-13";
const NAME_ISO_8859_14: &str = "iso-8859-14";
const NAME_ISO_8859_15: &str = "iso-8859-15";
const NAME_ISO_8859_16: &str = "iso-8859-16";
const NAME_KOI8_R: &str = "koi8-r";
const NAME_KOI8_U: &str = "koi8-u";
const NAME_MACINTOSH: &str = "macintosh";
const NAME_WINDOWS_874: &str = "windows-874";
const NAME_WINDOWS_1250: &str = "windows-1250";
const NAME_WINDOWS_1251: &str = "windows-1251";
const NAME_WINDOWS_1252: &str = "windows-1252";
const NAME_WINDOWS_1253: &str = "windows-1253";
const NAME_WINDOWS_1254: &str = "windows-1254";
const NAME_WINDOWS_1255: &str = "windows-1255";
const NAME_WINDOWS_1256: &str = "windows-1256";
const NAME_WINDOWS_1257: &str = "windows-1257";
const NAME_WINDOWS_1258: &str = "windows-1258";
const NAME_X_MAX_CYRILLIC: &str = "x-mac-cyrillic";
const NAME_GBK: &str = "gbk";
const NAME_GB18030: &str = "gb18030";
const NAME_HZGB2312: &str = "hz-gb-2312";
const NAME_BIG5: &str = "big5";
const NAME_EUC_JP: &str = "euc-jp";
const NAME_ISO_2022_JP: &str = "iso-2022-jp";
const NAME_SHIFT_JIS: &str = "shift_jis";
const NAME_EUC_KR: &str = "euc-kr";
const NAME_REPLACEMENT: &str = "replacement";
const NAME_UTF16_BE: &str = "utf-16be";
const NAME_UTF16_LE: &str = "utf-16le";
const NAME_X_USER_DEFINED: &str = "x-user-defined";

pub struct EncodingName {
    pub e: &'static encoding_rs::Encoding,
    pub name: &'static str,
}

lazy_static! {
    static ref encodings: HashMap<String, EncodingName> = {
        let mut m = HashMap::new();
        m.insert(
            "unicode-1-1-utf-8".to_string(),
            EncodingName {
                e: encoding_rs::UTF_8,
                name: NAME_UTF8,
            },
        );
        m.insert(
            "utf-8".to_string(),
            EncodingName {
                e: encoding_rs::UTF_8,
                name: NAME_UTF8,
            },
        );
        m.insert(
            "utf8".to_string(),
            EncodingName {
                e: encoding_rs::UTF_8,
                name: NAME_UTF8,
            },
        );
        m.insert(
            "utf8mb4".to_string(),
            EncodingName {
                e: encoding_rs::UTF_8,
                name: NAME_UTF8,
            },
        );
        m.insert(
            "binary".to_string(),
            EncodingName {
                e: encoding_rs::UTF_8,
                name: NAME_BINARY,
            },
        );
        m.insert(
            "866".to_string(),
            EncodingName {
                e: encoding_rs::IBM866,
                name: NAME_IBM866,
            },
        );
        m.insert(
            "cp866".to_string(),
            EncodingName {
                e: encoding_rs::IBM866,
                name: NAME_IBM866,
            },
        );
        m.insert(
            "csibm866".to_string(),
            EncodingName {
                e: encoding_rs::IBM866,
                name: NAME_IBM866,
            },
        );
        m.insert(
            "ibm866".to_string(),
            EncodingName {
                e: encoding_rs::IBM866,
                name: NAME_IBM866,
            },
        );
        m.insert(
            "csisolatin2".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_2,
                name: NAME_ISO_8859_2,
            },
        );
        m.insert(
            "iso-8859-2".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_2,
                name: NAME_ISO_8859_2,
            },
        );
        m.insert(
            "iso-ir-101".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_2,
                name: NAME_ISO_8859_2,
            },
        );
        m.insert(
            "iso8859-2".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_2,
                name: NAME_ISO_8859_2,
            },
        );
        m.insert(
            "iso88592".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_2,
                name: NAME_ISO_8859_2,
            },
        );
        m.insert(
            "iso_8859-2:1987".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_2,
                name: NAME_ISO_8859_2,
            },
        );
        m.insert(
            "l2".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_2,
                name: NAME_ISO_8859_2,
            },
        );
        m.insert(
            "latin2".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_2,
                name: NAME_ISO_8859_2,
            },
        );
        m.insert(
            "csisolatin3".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_3,
                name: NAME_ISO_8859_3,
            },
        );
        m.insert(
            "iso-8859-3".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_3,
                name: NAME_ISO_8859_3,
            },
        );
        m.insert(
            "iso-ir-109".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_3,
                name: NAME_ISO_8859_3,
            },
        );
        m.insert(
            "iso8859-3".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_3,
                name: NAME_ISO_8859_3,
            },
        );
        m.insert(
            "iso88593".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_3,
                name: NAME_ISO_8859_3,
            },
        );
        m.insert(
            "iso_8859-3".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_3,
                name: NAME_ISO_8859_3,
            },
        );
        m.insert(
            "iso_8859-3:1988".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_3,
                name: NAME_ISO_8859_3,
            },
        );
        m.insert(
            "l3".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_3,
                name: NAME_ISO_8859_3,
            },
        );
        m.insert(
            "latin3".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_3,
                name: NAME_ISO_8859_3,
            },
        );
        m.insert(
            "csisolatin4".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_4,
                name: NAME_ISO_8859_4,
            },
        );
        m.insert(
            "iso-8859-4".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_4,
                name: NAME_ISO_8859_4,
            },
        );
        m.insert(
            "iso-ir-110".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_4,
                name: NAME_ISO_8859_4,
            },
        );
        m.insert(
            "iso8859-4".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_4,
                name: NAME_ISO_8859_4,
            },
        );
        m.insert(
            "iso88594".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_4,
                name: NAME_ISO_8859_4,
            },
        );
        m.insert(
            "iso_8859-4".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_4,
                name: NAME_ISO_8859_4,
            },
        );
        m.insert(
            "iso_8859-4:1988".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_4,
                name: NAME_ISO_8859_4,
            },
        );
        m.insert(
            "l4".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_4,
                name: NAME_ISO_8859_4,
            },
        );
        m.insert(
            "latin4".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_4,
                name: NAME_ISO_8859_4,
            },
        );
        m.insert(
            "csisolatincyrillic".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_5,
                name: NAME_ISO_8859_5,
            },
        );
        m.insert(
            "cyrillic".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_5,
                name: NAME_ISO_8859_5,
            },
        );
        m.insert(
            "iso-8859-5".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_5,
                name: NAME_ISO_8859_5,
            },
        );
        m.insert(
            "iso-ir-144".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_5,
                name: NAME_ISO_8859_5,
            },
        );
        m.insert(
            "iso8859-5".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_5,
                name: NAME_ISO_8859_5,
            },
        );
        m.insert(
            "iso88595".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_5,
                name: NAME_ISO_8859_5,
            },
        );
        m.insert(
            "iso_8859-5".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_5,
                name: NAME_ISO_8859_5,
            },
        );
        m.insert(
            "iso_8859-5:1988".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_5,
                name: NAME_ISO_8859_5,
            },
        );
        m.insert(
            "arabic".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "asmo-708".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "csiso88596e".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "csiso88596i".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "csisolatinarabic".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "ecma-114".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "iso-8859-6".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "iso-8859-6-e".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "iso-8859-6-i".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "iso-ir-127".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "iso8859-6".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "iso88596".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "iso_8859-6".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "iso_8859-6:1987".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_6,
                name: NAME_ISO_8859_6,
            },
        );
        m.insert(
            "csisolatingreek".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_7,
                name: NAME_ISO_8859_7,
            },
        );
        m.insert(
            "ecma-118".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_7,
                name: NAME_ISO_8859_7,
            },
        );
        m.insert(
            "elot_928".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_7,
                name: NAME_ISO_8859_7,
            },
        );
        m.insert(
            "greek".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_7,
                name: NAME_ISO_8859_7,
            },
        );
        m.insert(
            "greek8".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_7,
                name: NAME_ISO_8859_7,
            },
        );
        m.insert(
            "iso-8859-7".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_7,
                name: NAME_ISO_8859_7,
            },
        );
        m.insert(
            "iso-ir-126".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_7,
                name: NAME_ISO_8859_7,
            },
        );
        m.insert(
            "iso8859-7".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_7,
                name: NAME_ISO_8859_7,
            },
        );
        m.insert(
            "iso88597".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_7,
                name: NAME_ISO_8859_7,
            },
        );
        m.insert(
            "iso_8859-7".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_7,
                name: NAME_ISO_8859_7,
            },
        );
        m.insert(
            "iso_8859-7:1987".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_7,
                name: NAME_ISO_8859_7,
            },
        );
        m.insert(
            "sun_eu_greek".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_7,
                name: NAME_ISO_8859_7,
            },
        );
        m.insert(
            "csiso88598e".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8,
                name: NAME_ISO_8859_8,
            },
        );
        m.insert(
            "csisolatinhebrew".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8,
                name: NAME_ISO_8859_8,
            },
        );
        m.insert(
            "hebrew".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8,
                name: NAME_ISO_8859_8,
            },
        );
        m.insert(
            "iso-8859-8".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8,
                name: NAME_ISO_8859_8,
            },
        );
        m.insert(
            "iso-8859-8-e".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8,
                name: NAME_ISO_8859_8,
            },
        );
        m.insert(
            "iso-ir-138".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8,
                name: NAME_ISO_8859_8,
            },
        );
        m.insert(
            "iso8859-8".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8,
                name: NAME_ISO_8859_8,
            },
        );
        m.insert(
            "iso88598".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8,
                name: NAME_ISO_8859_8,
            },
        );
        m.insert(
            "iso_8859-8".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8,
                name: NAME_ISO_8859_8,
            },
        );
        m.insert(
            "iso_8859-8:1988".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8,
                name: NAME_ISO_8859_8,
            },
        );
        m.insert(
            "visual".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8,
                name: NAME_ISO_8859_8,
            },
        );
        m.insert(
            "csiso88598i".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8_I,
                name: NAME_ISO_8859_8_I,
            },
        );
        m.insert(
            "iso-8859-8-i".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8_I,
                name: NAME_ISO_8859_8_I,
            },
        );
        m.insert(
            "logical".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_8_I,
                name: NAME_ISO_8859_8_I,
            },
        );
        m.insert(
            "csisolatin6".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_10,
                name: NAME_ISO_8859_10,
            },
        );
        m.insert(
            "iso-8859-10".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_10,
                name: NAME_ISO_8859_10,
            },
        );
        m.insert(
            "iso-ir-157".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_10,
                name: NAME_ISO_8859_10,
            },
        );
        m.insert(
            "iso8859-10".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_10,
                name: NAME_ISO_8859_10,
            },
        );
        m.insert(
            "iso885910".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_10,
                name: NAME_ISO_8859_10,
            },
        );
        m.insert(
            "l6".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_10,
                name: NAME_ISO_8859_10,
            },
        );
        m.insert(
            "latin6".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_10,
                name: NAME_ISO_8859_10,
            },
        );
        m.insert(
            "iso-8859-13".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_13,
                name: NAME_ISO_8859_13,
            },
        );
        m.insert(
            "iso8859-13".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_13,
                name: NAME_ISO_8859_13,
            },
        );
        m.insert(
            "iso885913".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_13,
                name: NAME_ISO_8859_13,
            },
        );
        m.insert(
            "iso-8859-14".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_14,
                name: NAME_ISO_8859_14,
            },
        );
        m.insert(
            "iso8859-14".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_14,
                name: NAME_ISO_8859_14,
            },
        );
        m.insert(
            "iso885914".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_14,
                name: NAME_ISO_8859_14,
            },
        );
        m.insert(
            "csisolatin9".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_15,
                name: NAME_ISO_8859_15,
            },
        );
        m.insert(
            "iso-8859-15".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_15,
                name: NAME_ISO_8859_15,
            },
        );
        m.insert(
            "iso8859-15".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_15,
                name: NAME_ISO_8859_15,
            },
        );
        m.insert(
            "iso885915".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_15,
                name: NAME_ISO_8859_15,
            },
        );
        m.insert(
            "iso_8859-15".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_15,
                name: NAME_ISO_8859_15,
            },
        );
        m.insert(
            "l9".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_15,
                name: NAME_ISO_8859_15,
            },
        );
        m.insert(
            "iso-8859-16".to_string(),
            EncodingName {
                e: encoding_rs::ISO_8859_16,
                name: NAME_ISO_8859_16,
            },
        );
        m.insert(
            "cskoi8r".to_string(),
            EncodingName {
                e: encoding_rs::KOI8_R,
                name: NAME_KOI8_R,
            },
        );
        m.insert(
            "koi".to_string(),
            EncodingName {
                e: encoding_rs::KOI8_R,
                name: NAME_KOI8_R,
            },
        );
        m.insert(
            "koi8".to_string(),
            EncodingName {
                e: encoding_rs::KOI8_R,
                name: NAME_KOI8_R,
            },
        );
        m.insert(
            "koi8-r".to_string(),
            EncodingName {
                e: encoding_rs::KOI8_R,
                name: NAME_KOI8_R,
            },
        );
        m.insert(
            "koi8_r".to_string(),
            EncodingName {
                e: encoding_rs::KOI8_R,
                name: NAME_KOI8_R,
            },
        );
        m.insert(
            "koi8-u".to_string(),
            EncodingName {
                e: encoding_rs::KOI8_U,
                name: NAME_KOI8_U,
            },
        );
        m.insert(
            "csmacintosh".to_string(),
            EncodingName {
                e: encoding_rs::MACINTOSH,
                name: NAME_MACINTOSH,
            },
        );
        m.insert(
            "mac".to_string(),
            EncodingName {
                e: encoding_rs::MACINTOSH,
                name: NAME_MACINTOSH,
            },
        );
        m.insert(
            "macintosh".to_string(),
            EncodingName {
                e: encoding_rs::MACINTOSH,
                name: NAME_MACINTOSH,
            },
        );
        m.insert(
            "x-mac-roman".to_string(),
            EncodingName {
                e: encoding_rs::MACINTOSH,
                name: NAME_MACINTOSH,
            },
        );
        m.insert(
            "dos-874".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_874,
                name: NAME_WINDOWS_874,
            },
        );
        m.insert(
            "iso-8859-11".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_874,
                name: NAME_WINDOWS_874,
            },
        );
        m.insert(
            "iso8859-11".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_874,
                name: NAME_WINDOWS_874,
            },
        );
        m.insert(
            "iso885911".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_874,
                name: NAME_WINDOWS_874,
            },
        );
        m.insert(
            "tis-620".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_874,
                name: NAME_WINDOWS_874,
            },
        );
        m.insert(
            "windows-874".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_874,
                name: NAME_WINDOWS_874,
            },
        );
        m.insert(
            "cp1250".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1250,
                name: NAME_WINDOWS_1250,
            },
        );
        m.insert(
            "windows-1250".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1250,
                name: NAME_WINDOWS_1250,
            },
        );
        m.insert(
            "x-cp1250".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1250,
                name: NAME_WINDOWS_1250,
            },
        );
        m.insert(
            "cp1251".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1251,
                name: NAME_WINDOWS_1251,
            },
        );
        m.insert(
            "windows-1251".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1251,
                name: NAME_WINDOWS_1251,
            },
        );
        m.insert(
            "x-cp1251".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1251,
                name: NAME_WINDOWS_1251,
            },
        );
        m.insert(
            "ansi_x3.4-1968".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "ascii".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "cp1252".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "cp819".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "csisolatin1".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "ibm819".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "iso-8859-1".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "iso-ir-100".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "iso8859-1".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "iso88591".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "iso_8859-1".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "iso_8859-1:1987".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "l1".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "latin1".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "us-ascii".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "windows-1252".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "x-cp1252".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1252,
                name: NAME_WINDOWS_1252,
            },
        );
        m.insert(
            "cp1253".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1253,
                name: NAME_WINDOWS_1253,
            },
        );
        m.insert(
            "windows-1253".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1253,
                name: NAME_WINDOWS_1253,
            },
        );
        m.insert(
            "x-cp1253".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1253,
                name: NAME_WINDOWS_1253,
            },
        );
        m.insert(
            "cp1254".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1254,
                name: NAME_WINDOWS_1254,
            },
        );
        m.insert(
            "csisolatin5".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1254,
                name: NAME_WINDOWS_1254,
            },
        );
        m.insert(
            "iso-8859-9".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1254,
                name: NAME_WINDOWS_1254,
            },
        );
        m.insert(
            "iso-ir-148".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1254,
                name: NAME_WINDOWS_1254,
            },
        );
        m.insert(
            "iso8859-9".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1254,
                name: NAME_WINDOWS_1254,
            },
        );
        m.insert(
            "iso88599".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1254,
                name: NAME_WINDOWS_1254,
            },
        );
        m.insert(
            "iso_8859-9".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1254,
                name: NAME_WINDOWS_1254,
            },
        );
        m.insert(
            "iso_8859-9:1989".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1254,
                name: NAME_WINDOWS_1254,
            },
        );
        m.insert(
            "l5".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1254,
                name: NAME_WINDOWS_1254,
            },
        );
        m.insert(
            "latin5".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1254,
                name: NAME_WINDOWS_1254,
            },
        );
        m.insert(
            "windows-1254".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1254,
                name: NAME_WINDOWS_1254,
            },
        );
        m.insert(
            "x-cp1254".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1254,
                name: NAME_WINDOWS_1254,
            },
        );
        m.insert(
            "cp1255".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1255,
                name: NAME_WINDOWS_1255,
            },
        );
        m.insert(
            "windows-1255".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1255,
                name: NAME_WINDOWS_1255,
            },
        );
        m.insert(
            "x-cp1255".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1255,
                name: NAME_WINDOWS_1255,
            },
        );
        m.insert(
            "cp1256".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1256,
                name: NAME_WINDOWS_1256,
            },
        );
        m.insert(
            "windows-1256".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1256,
                name: NAME_WINDOWS_1256,
            },
        );
        m.insert(
            "x-cp1256".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1256,
                name: NAME_WINDOWS_1256,
            },
        );
        m.insert(
            "cp1257".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1257,
                name: NAME_WINDOWS_1257,
            },
        );
        m.insert(
            "windows-1257".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1257,
                name: NAME_WINDOWS_1257,
            },
        );
        m.insert(
            "x-cp1257".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1257,
                name: NAME_WINDOWS_1257,
            },
        );
        m.insert(
            "cp1258".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1258,
                name: NAME_WINDOWS_1258,
            },
        );
        m.insert(
            "windows-1258".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1258,
                name: NAME_WINDOWS_1258,
            },
        );
        m.insert(
            "x-cp1258".to_string(),
            EncodingName {
                e: encoding_rs::WINDOWS_1258,
                name: NAME_WINDOWS_1258,
            },
        );
        m.insert(
            "x-mac-cyrillic".to_string(),
            EncodingName {
                e: encoding_rs::X_MAC_CYRILLIC,
                name: NAME_X_MAX_CYRILLIC,
            },
        );
        m.insert(
            "x-mac-ukrainian".to_string(),
            EncodingName {
                e: encoding_rs::X_MAC_CYRILLIC,
                name: NAME_X_MAX_CYRILLIC,
            },
        );
        m.insert(
            "chinese".to_string(),
            EncodingName {
                e: encoding_rs::GBK,
                name: NAME_GBK,
            },
        );
        m.insert(
            "csgb2312".to_string(),
            EncodingName {
                e: encoding_rs::GBK,
                name: NAME_GBK,
            },
        );
        m.insert(
            "csiso58gb231280".to_string(),
            EncodingName {
                e: encoding_rs::GBK,
                name: NAME_GBK,
            },
        );
        m.insert(
            "gb2312".to_string(),
            EncodingName {
                e: encoding_rs::GBK,
                name: NAME_GBK,
            },
        );
        m.insert(
            "gb_2312".to_string(),
            EncodingName {
                e: encoding_rs::GBK,
                name: NAME_GBK,
            },
        );
        m.insert(
            "gb_2312-80".to_string(),
            EncodingName {
                e: encoding_rs::GBK,
                name: NAME_GBK,
            },
        );
        m.insert(
            "gbk".to_string(),
            EncodingName {
                e: encoding_rs::GBK,
                name: NAME_GBK,
            },
        );
        m.insert(
            "iso-ir-58".to_string(),
            EncodingName {
                e: encoding_rs::GBK,
                name: NAME_GBK,
            },
        );
        m.insert(
            "x-gbk".to_string(),
            EncodingName {
                e: encoding_rs::GBK,
                name: NAME_GBK,
            },
        );
        m.insert(
            "gb18030".to_string(),
            EncodingName {
                e: encoding_rs::GB18030,
                name: NAME_GB18030,
            },
        );
        m.insert(
            "hz-gb-2312".to_string(),
            EncodingName {
                e: encoding_rs::GBK,
                name: NAME_HZGB2312,
            },
        );
        m.insert(
            "big5".to_string(),
            EncodingName {
                e: encoding_rs::BIG5,
                name: NAME_BIG5,
            },
        );
        m.insert(
            "big5-hkscs".to_string(),
            EncodingName {
                e: encoding_rs::BIG5,
                name: NAME_BIG5,
            },
        );
        m.insert(
            "cn-big5".to_string(),
            EncodingName {
                e: encoding_rs::BIG5,
                name: NAME_BIG5,
            },
        );
        m.insert(
            "csbig5".to_string(),
            EncodingName {
                e: encoding_rs::BIG5,
                name: NAME_BIG5,
            },
        );
        m.insert(
            "x-x-big5".to_string(),
            EncodingName {
                e: encoding_rs::BIG5,
                name: NAME_BIG5,
            },
        );
        m.insert(
            "cseucpkdfmtjapanese".to_string(),
            EncodingName {
                e: encoding_rs::EUC_JP,
                name: NAME_EUC_JP,
            },
        );
        m.insert(
            "euc-jp".to_string(),
            EncodingName {
                e: encoding_rs::EUC_JP,
                name: NAME_EUC_JP,
            },
        );
        m.insert(
            "x-euc-jp".to_string(),
            EncodingName {
                e: encoding_rs::EUC_JP,
                name: NAME_EUC_JP,
            },
        );
        m.insert(
            "csiso2022jp".to_string(),
            EncodingName {
                e: encoding_rs::ISO_2022_JP,
                name: NAME_ISO_2022_JP,
            },
        );
        m.insert(
            "iso-2022-jp".to_string(),
            EncodingName {
                e: encoding_rs::ISO_2022_JP,
                name: NAME_ISO_2022_JP,
            },
        );
        m.insert(
            "csshiftjis".to_string(),
            EncodingName {
                e: encoding_rs::SHIFT_JIS,
                name: NAME_SHIFT_JIS,
            },
        );
        m.insert(
            "ms_kanji".to_string(),
            EncodingName {
                e: encoding_rs::SHIFT_JIS,
                name: NAME_SHIFT_JIS,
            },
        );
        m.insert(
            "shift-jis".to_string(),
            EncodingName {
                e: encoding_rs::SHIFT_JIS,
                name: NAME_SHIFT_JIS,
            },
        );
        m.insert(
            "shift_jis".to_string(),
            EncodingName {
                e: encoding_rs::SHIFT_JIS,
                name: NAME_SHIFT_JIS,
            },
        );
        m.insert(
            "sjis".to_string(),
            EncodingName {
                e: encoding_rs::SHIFT_JIS,
                name: NAME_SHIFT_JIS,
            },
        );
        m.insert(
            "windows-31j".to_string(),
            EncodingName {
                e: encoding_rs::SHIFT_JIS,
                name: NAME_SHIFT_JIS,
            },
        );
        m.insert(
            "x-sjis".to_string(),
            EncodingName {
                e: encoding_rs::SHIFT_JIS,
                name: NAME_SHIFT_JIS,
            },
        );
        m.insert(
            "cseuckr".to_string(),
            EncodingName {
                e: encoding_rs::EUC_KR,
                name: NAME_EUC_KR,
            },
        );
        m.insert(
            "csksc56011987".to_string(),
            EncodingName {
                e: encoding_rs::EUC_KR,
                name: NAME_EUC_KR,
            },
        );
        m.insert(
            "euc-kr".to_string(),
            EncodingName {
                e: encoding_rs::EUC_KR,
                name: NAME_EUC_KR,
            },
        );
        m.insert(
            "iso-ir-149".to_string(),
            EncodingName {
                e: encoding_rs::EUC_KR,
                name: NAME_EUC_KR,
            },
        );
        m.insert(
            "korean".to_string(),
            EncodingName {
                e: encoding_rs::EUC_KR,
                name: NAME_EUC_KR,
            },
        );
        m.insert(
            "ks_c_5601-1987".to_string(),
            EncodingName {
                e: encoding_rs::EUC_KR,
                name: NAME_EUC_KR,
            },
        );
        m.insert(
            "ks_c_5601-1989".to_string(),
            EncodingName {
                e: encoding_rs::EUC_KR,
                name: NAME_EUC_KR,
            },
        );
        m.insert(
            "ksc5601".to_string(),
            EncodingName {
                e: encoding_rs::EUC_KR,
                name: NAME_EUC_KR,
            },
        );
        m.insert(
            "ksc_5601".to_string(),
            EncodingName {
                e: encoding_rs::EUC_KR,
                name: NAME_EUC_KR,
            },
        );
        m.insert(
            "windows-949".to_string(),
            EncodingName {
                e: encoding_rs::EUC_KR,
                name: NAME_EUC_KR,
            },
        );
        m.insert(
            "csiso2022kr".to_string(),
            EncodingName {
                e: encoding_rs::REPLACEMENT,
                name: NAME_REPLACEMENT,
            },
        );
        m.insert(
            "iso-2022-kr".to_string(),
            EncodingName {
                e: encoding_rs::REPLACEMENT,
                name: NAME_REPLACEMENT,
            },
        );
        m.insert(
            "iso-2022-cn".to_string(),
            EncodingName {
                e: encoding_rs::REPLACEMENT,
                name: NAME_REPLACEMENT,
            },
        );
        m.insert(
            "iso-2022-cn-ext".to_string(),
            EncodingName {
                e: encoding_rs::REPLACEMENT,
                name: NAME_REPLACEMENT,
            },
        );
        m.insert(
            "utf-16be".to_string(),
            EncodingName {
                e: encoding_rs::UTF_16BE,
                name: NAME_UTF16_BE,
            },
        );
        m.insert(
            "utf-16".to_string(),
            EncodingName {
                e: encoding_rs::UTF_16LE,
                name: NAME_UTF16_LE,
            },
        );
        m.insert(
            "utf-16le".to_string(),
            EncodingName {
                e: encoding_rs::UTF_16LE,
                name: NAME_UTF16_LE,
            },
        );
        m.insert(
            "x-user-defined".to_string(),
            EncodingName {
                e: encoding_rs::X_USER_DEFINED,
                name: NAME_X_USER_DEFINED,
            },
        );

        m
    };
}

#[allow(dead_code)]
pub fn lookup(label: &str) -> Option<&'static EncodingName> {
    let key = label
        .trim_matches(&['\t', '\n', '\t', char::from(12), ' '])
        .to_string();
    encodings.get(&key)
}
