use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

// PadSpace is to mark that trailing spaces are insignificant in comparisons
pub const PAD_SPACE: &str = "PAD SPACE";
// PadNone is to mark that trailing spaces are significant in comparisons
pub const PAD_NONE: &str = "NO PAD";

// Charset is a charset.
// Now we only support MySQL.
#[derive(Debug, Clone)]
pub struct Charset {
    pub name: String,
    pub default_collation: String,
    pub collations: HashMap<String, Rc<RefCell<Collation>>>,
    pub desc: String,
    pub maxlen: isize,
}

impl Charset {
    pub fn new(
        name: &str,
        maxlen: isize,
        default_collation: &str,
        desc: &str,
        cs: HashMap<String, Rc<RefCell<Collation>>>,
    ) -> Self {
        Charset {
            name: name.to_string(),
            default_collation: default_collation.to_string(),
            collations: cs,
            desc: desc.to_string(),
            maxlen,
        }
    }

    pub fn new_ref(
        name: &str,
        maxlen: isize,
        default_collation: &str,
        desc: &str,
        cs: HashMap<String, Rc<RefCell<Collation>>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(
            name,
            maxlen,
            default_collation,
            desc,
            cs,
        )))
    }
}

// Collation is a collation.
// Now we only support MySQL.
#[derive(Debug, Clone)]
pub struct Collation {
    pub id: isize,
    pub charset_name: String,
    pub name: String,
    pub is_default: bool,
    pub sortlen: isize,
    pub pad_attribute: String,
}

impl Collation {
    pub fn new(
        id: isize,
        charset_name: &str,
        name: &str,
        is_default: bool,
        sortlen: isize,
        pad_attribute: &str,
    ) -> Self {
        Collation {
            id,
            charset_name: charset_name.to_string(),
            name: name.to_string(),
            is_default,
            sortlen,
            pad_attribute: pad_attribute.to_string(),
        }
    }

    pub fn new_ref(
        id: isize,
        charset_name: &str,
        name: &str,
        is_default: bool,
        sortlen: isize,
        pad_attribute: &str,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(
            id,
            charset_name,
            name,
            is_default,
            sortlen,
            pad_attribute,
        )))
    }
}

// CollationBin is the default collation for CharsetBin.
pub const COLLATION_BIN: &str = "binary";
// CollationUTF8 is the default collation for CharsetUTF8.
pub const COLLATION_UTF8: &str = "utf8_bin";
// CollationUTF8MB4 is the default collation for CharsetUTF8MB4.
pub const COLLATION_UTF8MB4: &str = "utf8mb4_bin";
// CollationASCII is the default collation for CharsetACSII.
pub const COLLATION_ASCII: &str = "ascii_bin";
// CollationLatin1 is the default collation for CharsetLatin1.
pub const COLLATION_LATIN1: &str = "latin1_bin";
// CollationGBKBin is the default collation for CharsetGBK when new collation is disabled.
pub const COLLATION_GBK_BIN: &str = "gbk_bin";
// CollationGBKChineseCI is the default collation for CharsetGBK when new collation is enabled.
pub const COLLATION_GBK_CHINESE_CI: &str = "gbk_chinese_ci";

// CharsetASCII is a subset of UTF8.
pub const CHARSET_ASCII: &str = "ascii";
// CharsetBin is used for marking binary charset.
pub const CHARSET_BIN: &str = "binary";
// CharsetLatin1 is a single byte charset.
pub const CHARSET_LATIN1: &str = "latin1";
// CharsetUTF8 is the default charset for string types.
pub const CHARSET_UTF8: &str = "utf8";
// CharsetUTF8MB3 is 3 bytes utf8, a MySQL legacy encoding. "utf8" and "utf8mb3" are aliases.
pub const CHARSET_UTF8MB3: &str = "utf8mb3";
// CharsetUTF8MB4 represents 4 bytes utf8, which works the same way as utf8 in Go.
pub const CHARSET_UTF8MB4: &str = "utf8mb4";
//revive:disable:exported
pub const CHARSET_ARMSCII8: &str = "armscii8";
pub const CHARSET_BIG5: &str = "big5";
pub const CHARSET_CP1250: &str = "cp1250";
pub const CHARSET_CP1251: &str = "cp1251";
pub const CHARSET_CP1256: &str = "cp1256";
pub const CHARSET_CP1257: &str = "cp1257";
pub const CHARSET_CP850: &str = "cp850";
pub const CHARSET_CP852: &str = "cp852";
pub const CHARSET_CP866: &str = "cp866";
pub const CHARSET_CP932: &str = "cp932";
pub const CHARSET_DEC8: &str = "dec8";
pub const CHARSET_EUCJPMS: &str = "eucjpms";
pub const CHARSET_EUCKR: &str = "euckr";
pub const CHARSET_GB18030: &str = "gb18030";
pub const CHARSET_GB2312: &str = "gb2312";
pub const CHARSET_GBK: &str = "gbk";
pub const CHARSET_GEOSTD8: &str = "geostd8";
pub const CHARSET_GREEK: &str = "greek";
pub const CHARSET_HEBREW: &str = "hebrew";
pub const CHARSET_HP8: &str = "hp8";
pub const CHARSET_KEYBCS2: &str = "keybcs2";
pub const CHARSET_KOI8_R: &str = "koi8r";
pub const CHARSET_KOI8_U: &str = "koi8u";
pub const CHARSET_LATIN2: &str = "latin2";
pub const CHARSET_LATIN5: &str = "latin5";
pub const CHARSET_LATIN7: &str = "latin7";
pub const CHARSET_MAC_CE: &str = "macce";
pub const CHARSET_MAC_ROMAN: &str = "macroman";
pub const CHARSET_SJIS: &str = "sjis";
pub const CHARSET_SWE7: &str = "swe7";
pub const CHARSET_TIS620: &str = "tis620";
pub const CHARSET_UCS2: &str = "ucs2";
pub const CHARSET_UJIS: &str = "ujis";
pub const CHARSET_UTF16: &str = "utf16";
pub const CHARSET_UTF16_LE: &str = "utf16le";
pub const CHARSET_UTF32: &str = "utf32";
//revive:enable:exported

lazy_static! {
    pub static ref supported_collation_names: HashSet<String> = {
        let mut s = HashSet::<String>::new();
        s.insert(COLLATION_UTF8.to_string());
        s.insert(COLLATION_UTF8MB4.to_string());
        s.insert(COLLATION_ASCII.to_string());
        s.insert(COLLATION_LATIN1.to_string());
        s.insert(COLLATION_BIN.to_string());
        s.insert(COLLATION_GBK_BIN.to_string());

        s
    };
    pub static ref tiflash_supported_charsets: HashSet<String> = {
        let mut s = HashSet::<String>::new();
        s.insert(CHARSET_UTF8.to_string());
        s.insert(CHARSET_UTF8MB4.to_string());
        s.insert(CHARSET_ASCII.to_string());
        s.insert(CHARSET_LATIN1.to_string());
        s.insert(CHARSET_BIN.to_string());

        s
    };

    pub static ref collations: Vec<Rc<RefCell<Collation>>> = {
        let mut c = Vec::<Rc<RefCell<Collation>>>::new();
        c.push(Collation::new_ref(
            1,
            "big5",
            "big5_chinese_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            2,
            "latin2",
            "latin2_czech_cs",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            3,
            "dec8",
            "dec8_swedish_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            4,
            "cp850",
            "cp850_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            5,
            "latin1",
            "latin1_german1_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            6,
            "hp8",
            "hp8_english_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            7,
            "koi8r",
            "koi8r_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            8,
            "latin1",
            "latin1_swedish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            9,
            "latin2",
            "latin2_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            10,
            "swe7",
            "swe7_swedish_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            11,
            "ascii",
            "ascii_general_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            12,
            "ujis",
            "ujis_japanese_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            13,
            "sjis",
            "sjis_japanese_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            14,
            "cp1251",
            "cp1251_bulgarian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            15,
            "latin1",
            "latin1_danish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            16,
            "hebrew",
            "hebrew_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            18,
            "tis620",
            "tis620_thai_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            19,
            "euckr",
            "euckr_korean_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            20,
            "latin7",
            "latin7_estonian_cs",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            21,
            "latin2",
            "latin2_hungarian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            22,
            "koi8u",
            "koi8u_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            23,
            "cp1251",
            "cp1251_ukrainian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            24,
            "gb2312",
            "gb2312_chinese_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            25,
            "greek",
            "greek_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            26,
            "cp1250",
            "cp1250_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            27,
            "latin2",
            "latin2_croatian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            28,
            "gbk",
            "gbk_chinese_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            29,
            "cp1257",
            "cp1257_lithuanian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            30,
            "latin5",
            "latin5_turkish_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            31,
            "latin1",
            "latin1_german2_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            32,
            "armscii8",
            "armscii8_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            33,
            "utf8",
            "utf8_general_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            34,
            "cp1250",
            "cp1250_czech_cs",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            35,
            "ucs2",
            "ucs2_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            36,
            "cp866",
            "cp866_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            37,
            "keybcs2",
            "keybcs2_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            38,
            "macce",
            "macce_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            39,
            "macroman",
            "macroman_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            40,
            "cp852",
            "cp852_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            41,
            "latin7",
            "latin7_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            42,
            "latin7",
            "latin7_general_cs",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            43,
            "macce",
            "macce_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            44,
            "cp1250",
            "cp1250_croatian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            45,
            "utf8mb4",
            "utf8mb4_general_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            46,
            "utf8mb4",
            "utf8mb4_bin",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            47,
            "latin1",
            "latin1_bin",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            48,
            "latin1",
            "latin1_general_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            49,
            "latin1",
            "latin1_general_cs",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            50,
            "cp1251",
            "cp1251_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            51,
            "cp1251",
            "cp1251_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            52,
            "cp1251",
            "cp1251_general_cs",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            53,
            "macroman",
            "macroman_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            54,
            "utf16",
            "utf16_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            55,
            "utf16",
            "utf16_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            56,
            "utf16le",
            "utf16le_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            57,
            "cp1256",
            "cp1256_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            58,
            "cp1257",
            "cp1257_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            59,
            "cp1257",
            "cp1257_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            60,
            "utf32",
            "utf32_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            61,
            "utf32",
            "utf32_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            62,
            "utf16le",
            "utf16le_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(63, "binary", "binary", true, 1, PAD_NONE));
        c.push(Collation::new_ref(
            64,
            "armscii8",
            "armscii8_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(65, "ascii", "ascii_bin", true, 1, PAD_SPACE));
        c.push(Collation::new_ref(
            66,
            "cp1250",
            "cp1250_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            67,
            "cp1256",
            "cp1256_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            68,
            "cp866",
            "cp866_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(69, "dec8", "dec8_bin", false, 1, PAD_SPACE));
        c.push(Collation::new_ref(
            70,
            "greek",
            "greek_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            71,
            "hebrew",
            "hebrew_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(72, "hp8", "hp8_bin", false, 1, PAD_SPACE));
        c.push(Collation::new_ref(
            73,
            "keybcs2",
            "keybcs2_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            74,
            "koi8r",
            "koi8r_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            75,
            "koi8u",
            "koi8u_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            76,
            "utf8",
            "utf8_tolower_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            77,
            "latin2",
            "latin2_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            78,
            "latin5",
            "latin5_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            79,
            "latin7",
            "latin7_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            80,
            "cp850",
            "cp850_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            81,
            "cp852",
            "cp852_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(82, "swe7", "swe7_bin", false, 1, PAD_SPACE));
        c.push(Collation::new_ref(83, "utf8", "utf8_bin", true, 1, PAD_SPACE));
        c.push(Collation::new_ref(84, "big5", "big5_bin", false, 1, PAD_SPACE));
        c.push(Collation::new_ref(
            85,
            "euckr",
            "euckr_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            86,
            "gb2312",
            "gb2312_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(87, "gbk", "gbk_bin", true, 1, PAD_SPACE));
        c.push(Collation::new_ref(88, "sjis", "sjis_bin", false, 1, PAD_SPACE));
        c.push(Collation::new_ref(
            89,
            "tis620",
            "tis620_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(90, "ucs2", "ucs2_bin", false, 1, PAD_SPACE));
        c.push(Collation::new_ref(91, "ujis", "ujis_bin", false, 1, PAD_SPACE));
        c.push(Collation::new_ref(
            92,
            "geostd8",
            "geostd8_general_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            93,
            "geostd8",
            "geostd8_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            94,
            "latin1",
            "latin1_spanish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            95,
            "cp932",
            "cp932_japanese_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            96,
            "cp932",
            "cp932_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            97,
            "eucjpms",
            "eucjpms_japanese_ci",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            98,
            "eucjpms",
            "eucjpms_bin",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            99,
            "cp1250",
            "cp1250_polish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            101,
            "utf16",
            "utf16_unicode_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            102,
            "utf16",
            "utf16_icelandic_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            103,
            "utf16",
            "utf16_latvian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            104,
            "utf16",
            "utf16_romanian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            105,
            "utf16",
            "utf16_slovenian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            106,
            "utf16",
            "utf16_polish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            107,
            "utf16",
            "utf16_estonian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            108,
            "utf16",
            "utf16_spanish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            109,
            "utf16",
            "utf16_swedish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            110,
            "utf16",
            "utf16_turkish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            111,
            "utf16",
            "utf16_czech_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            112,
            "utf16",
            "utf16_danish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            113,
            "utf16",
            "utf16_lithuanian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            114,
            "utf16",
            "utf16_slovak_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            115,
            "utf16",
            "utf16_spanish2_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            116,
            "utf16",
            "utf16_roman_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            117,
            "utf16",
            "utf16_persian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            118,
            "utf16",
            "utf16_esperanto_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            119,
            "utf16",
            "utf16_hungarian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            120,
            "utf16",
            "utf16_sinhala_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            121,
            "utf16",
            "utf16_german2_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            122,
            "utf16",
            "utf16_croatian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            123,
            "utf16",
            "utf16_unicode_520_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            124,
            "utf16",
            "utf16_vietnamese_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            128,
            "ucs2",
            "ucs2_unicode_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            129,
            "ucs2",
            "ucs2_icelandic_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            130,
            "ucs2",
            "ucs2_latvian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            131,
            "ucs2",
            "ucs2_romanian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            132,
            "ucs2",
            "ucs2_slovenian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            133,
            "ucs2",
            "ucs2_polish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            134,
            "ucs2",
            "ucs2_estonian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            135,
            "ucs2",
            "ucs2_spanish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            136,
            "ucs2",
            "ucs2_swedish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            137,
            "ucs2",
            "ucs2_turkish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            138,
            "ucs2",
            "ucs2_czech_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            139,
            "ucs2",
            "ucs2_danish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            140,
            "ucs2",
            "ucs2_lithuanian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            141,
            "ucs2",
            "ucs2_slovak_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            142,
            "ucs2",
            "ucs2_spanish2_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            143,
            "ucs2",
            "ucs2_roman_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            144,
            "ucs2",
            "ucs2_persian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            145,
            "ucs2",
            "ucs2_esperanto_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            146,
            "ucs2",
            "ucs2_hungarian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            147,
            "ucs2",
            "ucs2_sinhala_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            148,
            "ucs2",
            "ucs2_german2_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            149,
            "ucs2",
            "ucs2_croatian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            150,
            "ucs2",
            "ucs2_unicode_520_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            151,
            "ucs2",
            "ucs2_vietnamese_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            159,
            "ucs2",
            "ucs2_general_mysql500_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            160,
            "utf32",
            "utf32_unicode_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            161,
            "utf32",
            "utf32_icelandic_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            162,
            "utf32",
            "utf32_latvian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            163,
            "utf32",
            "utf32_romanian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            164,
            "utf32",
            "utf32_slovenian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            165,
            "utf32",
            "utf32_polish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            166,
            "utf32",
            "utf32_estonian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            167,
            "utf32",
            "utf32_spanish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            168,
            "utf32",
            "utf32_swedish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            169,
            "utf32",
            "utf32_turkish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            170,
            "utf32",
            "utf32_czech_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            171,
            "utf32",
            "utf32_danish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            172,
            "utf32",
            "utf32_lithuanian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            173,
            "utf32",
            "utf32_slovak_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            174,
            "utf32",
            "utf32_spanish2_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            175,
            "utf32",
            "utf32_roman_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            176,
            "utf32",
            "utf32_persian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            177,
            "utf32",
            "utf32_esperanto_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            178,
            "utf32",
            "utf32_hungarian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            179,
            "utf32",
            "utf32_sinhala_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            180,
            "utf32",
            "utf32_german2_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            181,
            "utf32",
            "utf32_croatian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            182,
            "utf32",
            "utf32_unicode_520_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            183,
            "utf32",
            "utf32_vietnamese_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            192,
            "utf8",
            "utf8_unicode_ci",
            false,
            8,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            193,
            "utf8",
            "utf8_icelandic_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            194,
            "utf8",
            "utf8_latvian_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            195,
            "utf8",
            "utf8_romanian_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            196,
            "utf8",
            "utf8_slovenian_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            197,
            "utf8",
            "utf8_polish_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            198,
            "utf8",
            "utf8_estonian_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            199,
            "utf8",
            "utf8_spanish_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            200,
            "utf8",
            "utf8_swedish_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            201,
            "utf8",
            "utf8_turkish_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            202,
            "utf8",
            "utf8_czech_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            203,
            "utf8",
            "utf8_danish_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            204,
            "utf8",
            "utf8_lithuanian_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            205,
            "utf8",
            "utf8_slovak_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            206,
            "utf8",
            "utf8_spanish2_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            207,
            "utf8",
            "utf8_roman_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            208,
            "utf8",
            "utf8_persian_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            209,
            "utf8",
            "utf8_esperanto_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            210,
            "utf8",
            "utf8_hungarian_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            211,
            "utf8",
            "utf8_sinhala_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            212,
            "utf8",
            "utf8_german2_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            213,
            "utf8",
            "utf8_croatian_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            214,
            "utf8",
            "utf8_unicode_520_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            215,
            "utf8",
            "utf8_vietnamese_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            223,
            "utf8",
            "utf8_general_mysql500_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            224,
            "utf8mb4",
            "utf8mb4_unicode_ci",
            false,
            8,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            225,
            "utf8mb4",
            "utf8mb4_icelandic_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            226,
            "utf8mb4",
            "utf8mb4_latvian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            227,
            "utf8mb4",
            "utf8mb4_romanian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            228,
            "utf8mb4",
            "utf8mb4_slovenian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            229,
            "utf8mb4",
            "utf8mb4_polish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            230,
            "utf8mb4",
            "utf8mb4_estonian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            231,
            "utf8mb4",
            "utf8mb4_spanish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            232,
            "utf8mb4",
            "utf8mb4_swedish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            233,
            "utf8mb4",
            "utf8mb4_turkish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            234,
            "utf8mb4",
            "utf8mb4_czech_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            235,
            "utf8mb4",
            "utf8mb4_danish_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            236,
            "utf8mb4",
            "utf8mb4_lithuanian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            237,
            "utf8mb4",
            "utf8mb4_slovak_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            238,
            "utf8mb4",
            "utf8mb4_spanish2_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            239,
            "utf8mb4",
            "utf8mb4_roman_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            240,
            "utf8mb4",
            "utf8mb4_persian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            241,
            "utf8mb4",
            "utf8mb4_esperanto_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            242,
            "utf8mb4",
            "utf8mb4_hungarian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            243,
            "utf8mb4",
            "utf8mb4_sinhala_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            244,
            "utf8mb4",
            "utf8mb4_german2_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            245,
            "utf8mb4",
            "utf8mb4_croatian_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            246,
            "utf8mb4",
            "utf8mb4_unicode_520_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            247,
            "utf8mb4",
            "utf8mb4_vietnamese_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            248,
            "gb18030",
            "gb18030_chinese_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            249,
            "gb18030",
            "gb18030_bin",
            true,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            250,
            "gb18030",
            "gb18030_unicode_520_ci",
            false,
            1,
            PAD_SPACE,
        ));
        c.push(Collation::new_ref(
            255,
            "utf8mb4",
            "utf8mb4_0900_ai_ci",
            false,
            0,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            256,
            "utf8mb4",
            "utf8mb4_de_pb_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            257,
            "utf8mb4",
            "utf8mb4_is_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            258,
            "utf8mb4",
            "utf8mb4_lv_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            259,
            "utf8mb4",
            "utf8mb4_ro_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            260,
            "utf8mb4",
            "utf8mb4_sl_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            261,
            "utf8mb4",
            "utf8mb4_pl_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            262,
            "utf8mb4",
            "utf8mb4_et_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            263,
            "utf8mb4",
            "utf8mb4_es_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            264,
            "utf8mb4",
            "utf8mb4_sv_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            265,
            "utf8mb4",
            "utf8mb4_tr_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            266,
            "utf8mb4",
            "utf8mb4_cs_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            267,
            "utf8mb4",
            "utf8mb4_da_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            268,
            "utf8mb4",
            "utf8mb4_lt_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            269,
            "utf8mb4",
            "utf8mb4_sk_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            270,
            "utf8mb4",
            "utf8mb4_es_trad_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            271,
            "utf8mb4",
            "utf8mb4_la_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            273,
            "utf8mb4",
            "utf8mb4_eo_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            274,
            "utf8mb4",
            "utf8mb4_hu_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            275,
            "utf8mb4",
            "utf8mb4_hr_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            277,
            "utf8mb4",
            "utf8mb4_vi_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            278,
            "utf8mb4",
            "utf8mb4_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            279,
            "utf8mb4",
            "utf8mb4_de_pb_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            280,
            "utf8mb4",
            "utf8mb4_is_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            281,
            "utf8mb4",
            "utf8mb4_lv_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            282,
            "utf8mb4",
            "utf8mb4_ro_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            283,
            "utf8mb4",
            "utf8mb4_sl_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            284,
            "utf8mb4",
            "utf8mb4_pl_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            285,
            "utf8mb4",
            "utf8mb4_et_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            286,
            "utf8mb4",
            "utf8mb4_es_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            287,
            "utf8mb4",
            "utf8mb4_sv_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            288,
            "utf8mb4",
            "utf8mb4_tr_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            289,
            "utf8mb4",
            "utf8mb4_cs_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            290,
            "utf8mb4",
            "utf8mb4_da_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            291,
            "utf8mb4",
            "utf8mb4_lt_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            292,
            "utf8mb4",
            "utf8mb4_sk_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            293,
            "utf8mb4",
            "utf8mb4_es_trad_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            294,
            "utf8mb4",
            "utf8mb4_la_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            296,
            "utf8mb4",
            "utf8mb4_eo_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            297,
            "utf8mb4",
            "utf8mb4_hu_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            298,
            "utf8mb4",
            "utf8mb4_hr_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            300,
            "utf8mb4",
            "utf8mb4_vi_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            303,
            "utf8mb4",
            "utf8mb4_ja_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            304,
            "utf8mb4",
            "utf8mb4_ja_0900_as_cs_ks",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            305,
            "utf8mb4",
            "utf8mb4_0900_as_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            306,
            "utf8mb4",
            "utf8mb4_ru_0900_ai_ci",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            307,
            "utf8mb4",
            "utf8mb4_ru_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            308,
            "utf8mb4",
            "utf8mb4_zh_0900_as_cs",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            309,
            "utf8mb4",
            "utf8mb4_0900_bin",
            false,
            1,
            PAD_NONE,
        ));
        c.push(Collation::new_ref(
            2048,
            "utf8mb4",
            "utf8mb4_zh_pinyin_tidb_as_cs",
            false,
            1,
            PAD_NONE,
        ));

        c
    };

    pub static ref character_set_infos: HashMap<String, Rc<RefCell<Charset>>> = {
        let mut m = HashMap::<String, Rc<RefCell<Charset>>>::new();
        m.insert(
            CHARSET_UTF8.to_string(),
            Charset::new_ref(
                CHARSET_UTF8,
                3,
                COLLATION_UTF8,
                "UTF-8 Unicode",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_UTF8MB4.to_string(),
            Charset::new_ref(
                CHARSET_UTF8MB4,
                4,
                COLLATION_UTF8MB4,
                "UTF-8 Unicode",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_ASCII.to_string(),
            Charset::new_ref(
                CHARSET_ASCII,
                1,
                COLLATION_ASCII,
                "US ASCII",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_LATIN1.to_string(),
            Charset::new_ref(
                CHARSET_LATIN1,
                1,
                COLLATION_LATIN1,
                "Latin1",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_BIN.to_string(),
            Charset::new_ref(CHARSET_BIN, 1, COLLATION_BIN, "binary", HashMap::new()),
        );
        m.insert(
            CHARSET_GBK.to_string(),
            Charset::new_ref(
                CHARSET_GBK,
                2,
                COLLATION_GBK_BIN,
                "Chinese Internal Code Specification",
                HashMap::new(),
            ),
        );

        for c in collations.iter() {
            if let Some(charset) = m.get(&c.borrow().charset_name) {
                charset.borrow_mut().collations.insert(c.borrow().name.clone(), c.clone());
            }
        };

        m
    };

    pub static ref charsets: HashMap<String, Rc<RefCell<Charset>>> = {
        let mut m = HashMap::<String, Rc<RefCell<Charset>>>::new();
        m.insert(
            CHARSET_ARMSCII8.to_string(),
            Charset::new_ref (
                CHARSET_ARMSCII8,
                1,
                "armscii8_general_ci",
                "ARMSCII-8 Armenian",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_ASCII.to_string(),
            Charset::new_ref (
                CHARSET_ASCII,
                1,
                "ascii_general_ci",
                "US ASCII",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_BIG5.to_string(),
            Charset::new_ref (
                CHARSET_BIG5,
                2,
                "big5_chinese_ci",
                "Big5 Traditional Chinese",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_BIN.to_string(),
            Charset::new_ref (
                CHARSET_BIN,
                1,
                "binary",
                "Binary pseudo charset",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_LATIN1.to_string(),
            Charset::new_ref (
                CHARSET_LATIN1,
                1,
                "cp1250_general_ci",
                "Windows Central European",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_CP1250.to_string(),
            Charset::new_ref (
                CHARSET_CP1250,
                1,
                "cp1251_general_ci",
                "Windows Cyrillic",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_CP1251.to_string(),
            Charset::new_ref (
                CHARSET_CP1251,
                1,
                "cp1256_general_ci",
                "Windows Arabic",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_CP1256.to_string(),
            Charset::new_ref (
                CHARSET_CP1256,
                1,
                "cp1257_general_ci",
                "Windows Baltic",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_CP1257.to_string(),
            Charset::new_ref (
                CHARSET_CP1257,
                1,
                "cp850_general_ci",
                "DOS West European",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_CP850.to_string(),
            Charset::new_ref (
                CHARSET_CP850,
                1,
                "cp852_general_ci",
                "DOS Central European",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_CP852.to_string(),
            Charset::new_ref (
                CHARSET_CP852,
                1,
                "cp866_general_ci",
                "DOS Russian",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_CP866.to_string(),
            Charset::new_ref (
                CHARSET_CP866,
                1,
                "cp932_japanese_ci",
                "SJIS for Windows Japanese",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_CP932.to_string(),
            Charset::new_ref (
                CHARSET_CP932,
                2,
                "dec8_swedish_ci",
                "DEC West European",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_DEC8.to_string(),
            Charset::new_ref (
                CHARSET_DEC8,
                1,
                "eucjpms_japanese_ci",
                "UJIS for Windows Japanese",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_EUCJPMS.to_string(),
            Charset::new_ref (
                CHARSET_EUCJPMS,
                3,
                "euckr_korean_ci",
                "EUC-KR Korean",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_EUCKR.to_string(),
            Charset::new_ref (
                CHARSET_EUCKR,
                2,
                "gb18030_chinese_ci",
                "China National Standard GB18030",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_GB18030.to_string(),
            Charset::new_ref (
                CHARSET_GB18030,
                4,
                "gb2312_chinese_ci",
                "GB2312 Simplified Chinese",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_GB2312.to_string(),
            Charset::new_ref (
                CHARSET_GB2312,
                2,
                "gbk_chinese_ci",
                "GBK Simplified Chinese",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_GBK.to_string(),
            Charset::new_ref (
                CHARSET_GBK,
                2,
                "geostd8_general_ci",
                "GEOSTD8 Georgian",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_GEOSTD8.to_string(),
            Charset::new_ref (
                CHARSET_GEOSTD8,
                1,
                "greek_general_ci",
                "ISO 8859-7 Greek",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_GREEK.to_string(),
            Charset::new_ref (
                CHARSET_GREEK,
                1,
                "hebrew_general_ci",
                "ISO 8859-8 Hebrew",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_HEBREW.to_string(),
            Charset::new_ref (
                CHARSET_HEBREW,
                1,
                "hp8_english_ci",
                "HP West European",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_HP8.to_string(),
            Charset::new_ref (
                CHARSET_HP8,
                1,
                "keybcs2_general_ci",
                "DOS Kamenicky Czech-Slovak",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_KEYBCS2.to_string(),
            Charset::new_ref (
                CHARSET_KEYBCS2,
                1,
                "koi8r_general_ci",
                "KOI8-R Relcom Russian",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_KOI8_R.to_string(),
            Charset::new_ref (
                CHARSET_KOI8_R,
                1,
                "koi8u_general_ci",
                "KOI8-U Ukrainian",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_KOI8_U.to_string(),
            Charset::new_ref (
                CHARSET_KOI8_U,
                1,
                "latin1_swedish_ci",
                "cp1252 West European",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_LATIN2.to_string(),
            Charset::new_ref (
                CHARSET_LATIN2,
                1,
                "latin2_general_ci",
                "ISO 8859-2 Central European",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_LATIN5.to_string(),
            Charset::new_ref (
                CHARSET_LATIN5,
                1,
                "latin5_turkish_ci",
                "ISO 8859-9 Turkish",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_LATIN7.to_string(),
            Charset::new_ref (
                CHARSET_LATIN7,
                1,
                "latin7_general_ci",
                "ISO 8859-13 Baltic",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_MAC_CE.to_string(),
            Charset::new_ref (
                CHARSET_MAC_CE,
                1,
                "macce_general_ci",
                "Mac Central European",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_MAC_ROMAN.to_string(),
            Charset::new_ref (
                CHARSET_MAC_ROMAN,
                1,
                "macroman_general_ci",
                "Mac West European",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_SJIS.to_string(),
            Charset::new_ref (
                CHARSET_SJIS,
                2,
                "sjis_japanese_ci",
                "Shift-JIS Japanese",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_SWE7.to_string(),
            Charset::new_ref (
                CHARSET_SWE7,
                1,
                "swe7_swedish_ci",
                "7bit Swedish",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_TIS620.to_string(),
            Charset::new_ref (
                CHARSET_TIS620,
                1,
                "tis620_thai_ci",
                "TIS620 Thai",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_UCS2.to_string(),
            Charset::new_ref (
                CHARSET_UCS2,
                2,
                "ucs2_general_ci",
                "UCS-2 Unicode",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_UJIS.to_string(),
            Charset::new_ref (
                CHARSET_UJIS,
                3,
                "ujis_japanese_ci",
                "EUC-JP Japanese",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_UTF16.to_string(),
            Charset::new_ref (
                CHARSET_UTF16,
                4,
                "utf16_general_ci",
                "UTF-16 Unicode",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_UTF16_LE.to_string(),
            Charset::new_ref (
                CHARSET_UTF16_LE,
                4,
                "utf16le_general_ci",
                "UTF-16LE Unicode",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_UTF32.to_string(),
            Charset::new_ref (
                CHARSET_UTF32,
                4,
                "utf32_general_ci",
                "UTF-32 Unicode",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_UTF8.to_string(),
            Charset::new_ref (
                CHARSET_UTF8,
                3,
                "utf8_general_ci",
                "UTF-8 Unicode",
                HashMap::new(),
            ),
        );
        m.insert(
            CHARSET_UTF8MB4.to_string(),
            Charset::new_ref (
                CHARSET_UTF8MB4,
                4,
                "utf8mb4_0900_ai_ci",
                "UTF-8 Unicode",
                HashMap::new(),
            ),
        );

        for c in collations.iter() {
            if let Some(charset) = m.get(&c.borrow().charset_name) {
                charset.borrow_mut().collations.insert(c.borrow().name.clone(), c.clone());
            }
        };

        m
    };

    pub static ref collations_id_map: HashMap<isize, Rc<RefCell<Collation>>> = {
        let mut m: = HashMap::<isize,Rc<RefCell<Collation>>>::new();
        for c in collations.iter() {
            m.insert(c.borrow().id, c.clone());
        };

        m
    };

    pub static ref collations_name_map: HashMap<String, Rc<RefCell<Collation>>> = {
        let mut m: = HashMap::<String,Rc<RefCell<Collation>>>::new();
        for c in collations.iter() {
            m.insert(c.borrow().name.clone(), c.clone());
        };

        m
    };

    pub static ref supported_collations: Vec<Rc<RefCell<Collation>>> = {
        let mut v = Vec::<Rc<RefCell<Collation>>>::new();
        for c in collations.iter() {
            if supported_collation_names.get(&c.borrow().name).is_some() {
                v.push(c.clone());
            }
        }

        v
    };
}
