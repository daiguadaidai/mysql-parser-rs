#[cfg(test)]
mod tests {
    use crate::charset::charset;
    use crate::charset::encoding::{OP_DECODE, OP_DECODE_REPLACE, OP_ENCODE, OP_ENCODE_REPLACE};
    use crate::charset::encoding_lazy_static::encoding_gbk_impl;
    use crate::charset::encoding_table::lookup;
    use crate::charset::encoding_trait::EncodingTrait;
    use crate::common::error::CustomError;

    #[allow(dead_code)]
    #[derive(Debug)]
    struct EncodeCase {
        pub utf8_str: String,
        pub result: String,
        pub is_valid: bool,
    }

    #[test]
    fn test_encoding() -> Result<(), CustomError> {
        let enc = &encoding_gbk_impl;
        println!("{:?} {:?}", enc.name(), charset::CHARSET_GBK);
        assert_eq!(enc.name(), charset::CHARSET_GBK);

        let txt = "一二三四";
        let name = lookup("gbk").unwrap();
        let (gbk_encode_txt, _, _) = name.e.encode(txt);
        println!("txt           : {:?}", txt.as_bytes());
        println!("gbk_encode_txt: {:?}", &gbk_encode_txt);
        let result = enc.transform(&mut vec![], &gbk_encode_txt, OP_DECODE)?;
        println!("result txt    : {:?}", &result);
        assert_eq!(txt.as_bytes(), &result[..]);

        let gbk_encoded_txt2 = enc.transform(&mut vec![], txt.as_bytes(), OP_ENCODE)?;
        println!("gbk_encoded_txt2: {:?}", &gbk_encoded_txt2);
        assert_eq!(&gbk_encode_txt, &gbk_encoded_txt2);

        let result2 = enc.transform(&mut vec![], &gbk_encoded_txt2, OP_DECODE)?;
        println!("result2       : {:?}", &result2);
        assert_eq!(txt.as_bytes(), &result2);

        Ok(())
    }

    #[test]
    fn gbk_coding_test() -> Result<(), CustomError> {
        let enc = &encoding_gbk_impl;
        let mut gbk_cases = Vec::new();
        gbk_cases.push(EncodeCase {
            utf8_str: "一二三".to_string(),
            result: "涓?簩涓?".to_string(),
            is_valid: false,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "一二三123".to_string(),
            result: "涓?簩涓?123".to_string(),
            is_valid: false,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "测试".to_string(),
            result: "娴嬭瘯".to_string(),
            is_valid: true,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "案1案2".to_string(),
            result: "妗?妗?".to_string(),
            is_valid: false,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "案1案2".to_string(),
            result: "妗?妗?".to_string(),
            is_valid: false,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "焊䏷菡釬".to_string(),
            result: "鐒婁彿鑿￠嚞".to_string(),
            is_valid: true,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "鞍杏以伊位依".to_string(),
            result: "闉嶆潖浠ヤ紛浣嶄緷".to_string(),
            is_valid: true,
        });

        gbk_cases.push(EncodeCase {
            utf8_str: "移維緯胃萎衣謂違".to_string(),
            result: "绉荤董绶\u{e21d}儍钀庤。璎傞仌".to_string(),
            is_valid: true,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "仆仂仗仞仭仟价伉佚估".to_string(),
            result: "浠嗕粋浠椾粸浠浠\u{e15d}粺浠蜂級浣氫及".to_string(),
            is_valid: true,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "佝佗佇佶侈侏侘佻佩佰侑佯".to_string(),
            result: "浣濅綏浣囦蕉渚堜緩渚樹交浣╀桨渚戜蒋".to_string(),
            is_valid: true,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "".to_string(),
            result: "聙".to_string(),
            is_valid: true,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "a".to_string(),
            result: "聙a".to_string(),
            is_valid: true,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "aa".to_string(),
            result: "聙aa".to_string(),
            is_valid: true,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "aaab".to_string(),
            result: "aa聙ab".to_string(),
            is_valid: true,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "a你好a测试".to_string(),
            result: "a浣犲ソ聙a娴嬭瘯".to_string(),
            is_valid: true,
        });
        gbk_cases.push(EncodeCase {
            utf8_str: "aa".to_string(),
            result: "aa聙".to_string(),
            is_valid: true,
        });

        for (i, tc) in gbk_cases.iter().enumerate() {
            println!("gbk case [{}]: {:?}", i, &tc);
            let result2 = enc.transform(&mut vec![], tc.utf8_str.as_bytes(), OP_DECODE_REPLACE);
            if tc.is_valid {
                if result2.is_err() {
                    return Err(result2.unwrap_err());
                }
            } else {
                match result2 {
                    Ok(v) => {
                        println!("gbk case [{}]: {:?}", i, &v);
                        println!("gbk case [{}]: {:?}", i, std::str::from_utf8(&v).unwrap());
                        return Err(CustomError::Normal(format!(
                            "应该出错, 但是没有错. 数据: {:?}",
                            &tc
                        )));
                    }
                    Err(e) => {
                        println!("gbk case [{}]: {:?}", i, e.to_string())
                    }
                }
            }
        }

        Ok(())
    }

    #[test]
    fn utf8_encoding_test() -> Result<(), CustomError> {
        let enc = &encoding_gbk_impl;

        let mut utf8_cases = Vec::<EncodeCase>::new();
        utf8_cases.push(EncodeCase {
            utf8_str: "一二三".to_string(),
            result: "һ¶þÈý".to_string(),
            is_valid: true,
        });
        utf8_cases.push(EncodeCase {
            utf8_str: "🀁".to_string(),
            result: "?".to_string(),
            is_valid: false,
        });
        utf8_cases.push(EncodeCase {
            utf8_str: "valid_string_🀁".to_string(),
            result: "valid_string_?".to_string(),
            is_valid: false,
        });
        utf8_cases.push(EncodeCase {
            utf8_str: "€🀁".to_string(),
            result: "?".to_string(),
            is_valid: false,
        });
        utf8_cases.push(EncodeCase {
            utf8_str: "€a".to_string(),
            result: "?a".to_string(),
            is_valid: false,
        });
        utf8_cases.push(EncodeCase {
            utf8_str: "a€aa".to_string(),
            result: "a?aa".to_string(),
            is_valid: false,
        });
        utf8_cases.push(EncodeCase {
            utf8_str: "aaa€".to_string(),
            result: "aaa?".to_string(),
            is_valid: false,
        });

        for (i, tc) in utf8_cases.iter().enumerate() {
            println!("utf8 case [{}]: {:?}", i, &tc);
            let result2 = enc.transform(&mut vec![], tc.utf8_str.as_bytes(), OP_ENCODE_REPLACE);
            if tc.is_valid {
                if result2.is_err() {
                    return Err(result2.unwrap_err());
                }
            } else {
                match result2 {
                    Ok(v) => {
                        println!("utf8 case [{}]: {:?}", i, &v);
                        println!("utf8 case [{}]: {:?}", i, std::str::from_utf8(&v).unwrap());
                        return Err(CustomError::Normal(format!(
                            "应该出错, 但是没有错. 数据: {:?}",
                            &tc
                        )));
                    }
                    Err(e) => {
                        println!("utf8 case [{}]: {:?}", i, e.to_string())
                    }
                }
            }
        }

        Ok(())
    }
}
