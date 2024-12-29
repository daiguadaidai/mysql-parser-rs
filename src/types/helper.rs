pub fn is_space(c: u8) -> bool {
    c == ' ' as u8 || c == '\t' as u8
}

fn is_digit(c: u8) -> bool {
    c >= '0' as u8 && c <= '9' as u8
}

pub fn pow10(x: u32) -> u32 {
    10_u32.pow(x)
}

pub fn my_min(a: isize, b: isize) -> isize {
    if a < b {
        return a;
    }
    b
}
