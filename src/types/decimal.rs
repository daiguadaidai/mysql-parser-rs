use crate::common::error::CustomError;
use crate::types::helper::{is_space, my_min, pow10};
use logos::Source;
use nom::character::is_digit;
use std::fmt;

const PANIC_INFO: &str = "This branch is not implemented. This is because you are trying to test something specific to TiDB's MyDecimal implementation. It is recommended to do this in TiDB repository.";

// constant values.
const MAX_WORD_BUF_LEN: isize = 9; // A MyDecimal holds 9 words.
const DIGITS_PER_WORD: isize = 9; // A word holds 9 digits.
const DIG_MASK: isize = 100000000;
const WORD_BUF_LEN: isize = 9;

// fixWordCntError limits word count in wordBufLen, and returns overflow or truncate error.
fn fix_word_cnt_error(words_int: isize, words_frac: isize) -> Result<(isize, isize), CustomError> {
    if words_int + words_frac > WORD_BUF_LEN {
        panic!(PANIC_INFO)
    }
    Ok((words_int, words_frac))
}

/*
countLeadingZeroes returns the number of leading zeroes that can be removed from fraction.

@param   i    start index
@param   word value to compare against list of powers of 10
*/
fn count_leading_zeroes(mut i: u32, mut word: u32) -> isize {
    let mut leading = 0_isize;
    while word < pow10(i) {
        i -= 1;
        leading += 1;
    }

    leading
}

fn digits_to_words(digits: isize) -> isize {
    (digits + DIGITS_PER_WORD - 1) / DIGITS_PER_WORD
}

#[derive(Debug, Default)]
pub struct Decimal {
    pub digits_int: i8,  // the number of *decimal* digits before the point.
    pub digits_frac: i8, // the number of decimal digits after the point.
    pub result_frac: i8, // result fraction digits.
    pub negative: bool,
    // wordBuf is an array of int32 words.
    // A word is an int32 value can hold 9 digits.(0 <= word < wordBase)
    pub word_buf: [i32; 9],
}

// 为 MyStruct 实现 fmt::Display
impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = String::from_utf8(self.to_bytes()).unwrap();

        write!(f, "{}", s)
    }
}

impl Decimal {
    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self, CustomError> {
        let mut decimal = Decimal::default();

        let mut i = 0;
        while i < bytes.len() {
            if !is_space(bytes[i]) {
                bytes = &bytes[i..];
                break;
            }
            i += 1;
        }

        if bytes.len() == 0 {
            panic!(PANIC_INFO);
        }

        match bytes[0] as char {
            '-' => {
                decimal.negative = true;
                bytes = &bytes[1..];
            }
            '+' => {
                bytes = &bytes[1..];
            }
            _ => {}
        }

        let mut bytes_idx = 0_usize;
        while bytes_idx < bytes.len() && is_digit(bytes[bytes_idx]) {
            bytes_idx += 1
        }

        let mut digits_int = bytes_idx;
        let mut digits_frac = 0_usize;
        let mut end_idx = 0_usize;
        if bytes_idx < bytes.len() && bytes[bytes_idx] == '.' as u8 {
            end_idx = bytes_idx + 1;
            while end_idx < bytes.len() && is_digit(bytes[end_idx]) {
                end_idx += 1;
            }
            digits_frac = end_idx - bytes_idx - 1;
        } else {
            digits_frac = 0;
            end_idx = bytes_idx;
        }

        if digits_int + digits_frac == 0 {
            panic!(PANIC_INFO)
        }

        let words_int = digits_to_words(digits_int as isize);
        let words_frac = digits_to_words(digits_frac as isize);
        let (mut words_int, _) = fix_word_cnt_error(words_int, words_frac).unwrap();
        decimal.digits_int = digits_int as i8;
        decimal.digits_frac = digits_frac as i8;
        let mut word_idx = words_int;
        let mut str_idx_tmp = bytes_idx;
        let mut word = 0_i32;
        let mut inner_idx = 0_isize;
        while digits_int > 0 {
            digits_int -= 1;
            bytes_idx -= 1;
            word += (bytes[bytes_idx] - '0' as u8) as i32 * pow10(inner_idx as u32) as i32;
            inner_idx += 1;
            if inner_idx == DIGITS_PER_WORD {
                word_idx -= 1;
                decimal.word_buf[word_idx] = word;
                word = 0;
                inner_idx = 0;
            }
        }

        if inner_idx != 0 {
            word_idx -= 1;
            decimal.word_buf[word_idx] = word
        }

        word_idx = words_int;
        bytes_idx = str_idx_tmp;
        word = 0;
        inner_idx = 0;
        while digits_frac > 0 {
            digits_frac -= 1;
            bytes_idx += 1;
            word = (bytes[bytes_idx] - '0' as u8) as i32 + word * 10;
            inner_idx += 1;
            if inner_idx == DIGITS_PER_WORD {
                decimal.word_buf[word_idx] = word;
                word_idx += 1;
                word = 0;
                inner_idx = 0;
            }
        }

        if inner_idx != 0 {
            decimal.word_buf[word_idx] = word * pow10((DIGITS_PER_WORD - inner_idx) as u32) as i32
        }
        if end_idx + 1 <= bytes.len()
            && (bytes[end_idx] == 'e' as u8 || bytes[end_idx] == 'E' as u8)
        {
            panic!(PANIC_INFO)
        }
        let mut all_zero = true;
        let mut i = 0_isize;
        while i < WORD_BUF_LEN {
            if decimal.word_buf[i] != 0 {
                all_zero = false;
                break;
            }

            i += 1;
        }
        if all_zero {
            decimal.negative = false;
        }
        decimal.result_frac = decimal.digits_frac;

        Ok(decimal)
    }

    fn string_size(&self) -> isize {
        (self.digits_int + self.digits_frac + 3) as isize
    }

    fn remove_leading_zeros(&self) -> (isize, isize) {
        let mut digits_int = self.digits_int as isize;
        let mut word_idx = 0_isize;
        let mut i = ((digits_int - 1) % DIGITS_PER_WORD) + 1;

        while digits_int > 0 && self.word_buf[word_idx] == 0 {
            digits_int -= i;
            i = DIGITS_PER_WORD;
            word_idx += 1;
        }

        if digits_int > 0 {
            let i = ((digits_int - 1) % DIGITS_PER_WORD) as u32;
            digits_int -= count_leading_zeroes(i, self.word_buf[word_idx])
        } else {
            digits_int = 0
        }

        (digits_int, word_idx)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0_u8; self.string_size() as usize];
        let mut digits_frac = self.digits_frac as isize;
        let (mut word_start_idx, mut digits_int) = self.remove_leading_zeros();
        if digits_int + digits_frac == 0 {
            digits_int = 1;
            word_start_idx = 0;
        }

        let mut digits_int_len = digits_int;
        if digits_int_len == 0 {
            digits_int_len = 1
        }

        let digits_frac_len = digits_frac;
        let mut length = digits_int_len + digits_frac_len;
        if self.negative {
            length += 1;
        }
        if digits_frac > 0 {
            length += 1;
        }

        let bytes = bytes[..length];
        let mut bytes_idx = 0_isize;
        if self.negative {
            bytes[bytes_idx] = '-';
            bytes_idx += 1;
        }

        let mut fill = 0_isize;
        if digits_frac > 0 {
            let mut frac_idx = bytes_idx + digits_int_len;
            fill = digits_frac_len - digits_frac;
            let mut word_idx = word_start_idx + digits_to_words(digits_int);
            bytes[frac_idx] = '.';
            frac_idx += 1;
            while digits_frac > 0 {
                let mut x = self.word_buf[word_idx];
                word_idx += 1;

                let mut i = my_min(digits_frac, DIGITS_PER_WORD);
                while i > 0 {
                    let y = x / DIG_MASK;
                    bytes[frac_idx] = y as u8 + '0' as u8;
                    frac_idx += 1;
                    x -= y * DIG_MASK;
                    x *= 10;
                    i -= 1;
                }
                digits_frac -= DIGITS_PER_WORD;
            }
            while fill > 0 {
                bytes[frac_idx] = '0';
                frac_idx += 1;
                fill -= 1;
            }
        }

        fill = digits_int_len - digits_int;
        if digits_int == 0 {
            fill -= 1 /* symbol 0 before digital point */
        }

        while fill > 0 {
            bytes[bytes_idx] = '0';
            bytes_idx += 1;
            fill -= 1
        }

        if digits_int > 0 {
            bytes_idx += digits_int;
            let mut word_idx = word_start_idx + digits_to_words(digits_int);

            while digits_int > 0 {
                word_idx -= 1;
                let mut x = self.word_buf[word_idx];

                let mut i = my_min(digits_int, DIGITS_PER_WORD);
                while i > 0 {
                    let y = x / 10;
                    bytes_idx -= 1;
                    bytes[bytes_idx] = '0' as u8 + (x - y * 10) as u8;
                    x = y;
                    i -= 1;
                }
                digits_int -= DIGITS_PER_WORD;
            }
        } else {
            bytes[bytes_idx] = '0';
        }

        bytes
    }
}
