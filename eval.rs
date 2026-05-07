#![no_std]
#![no_main]
#![crate_type = "lib"]

use core::ffi::c_char;

static mut INPUT: *const c_char = core::ptr::null();
static mut POS: isize = 0;

unsafe fn peek_char() -> Option<u8> {
    let c = *INPUT.offset(POS) as u8;
    if c == 0 {
        return None;
    } else {
        return Some(c);
    }
}

unsafe fn next_char() -> Option<u8> {
    let c = peek_char();
    if c.is_some() {
        POS += 1;
    }
    return c;
}

unsafe fn skip_ws() {
    while let Some(c) = peek_char() {
        if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' {
            next_char();
        } else {
            break;
        }
    }
}

unsafe fn parse_number() -> i32 {
    skip_ws();
    let mut res = 0;
    while let Some(c) = peek_char() {
        if c >= b'0' && c <= b'9' {
            res = res * 10 + (c - b'0') as i32;
            next_char();
        } else {
            break;
        }
    }
    return res;
}

unsafe fn parse_term() -> i32 {
    let mut res = parse_number();
    loop {
        skip_ws();
        match peek_char() {
            Some(b'*') => {
                next_char();
                res *= parse_number();
            }
            Some(b'/') => {
                next_char();
                let divisor = parse_number();
                if divisor != 0 {
                    res /= divisor;
                }
            }
            _ => break,
        }
    }
    return res;
}

unsafe fn parse_expr() -> i32 {
    let mut res = parse_term();
    loop {
        skip_ws();
        match peek_char() {
            Some(b'+') => {
                next_char();
                res += parse_term();
            }
            Some(b'-') => {
                next_char();
                res -= parse_term();
            }
            _ => break,
        }
    }
    res
}

pub unsafe fn eval(s: *const c_char) -> i32 {
    INPUT = s;
    POS = 0;
    parse_expr()
}
