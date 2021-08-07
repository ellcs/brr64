use regex::Regex;

pub const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub const EQUALS : u8 = b'=';

enum Char64 {
    Sym,
    Real(u32)
}


enum OutChar64 {
    Equals,
    Single(u8),
    Multiple(Vec<u8>)
}

struct Candidates(Vec<OutChar64>, Vec<OutChar64>, Vec<OutChar64>);

impl From<&str> for Candidates {
    fn from(inp: &str) -> Self {
        panic!()
    }
}

impl From<Candidates> for Regex {
    fn from(outchars: Candidates) -> Self {
        panic!()
    }
}


fn base64_three_chars_symbolic(chars: &[Char64], result: &mut Vec<u8>) {
    match chars {
        [Char64::Real(c1), Char64::Real(c2), Char64::Real(c3)] => first_case(c1, c2, c3, result),
        [Char64::Sym,      Char64::Real(c2), Char64::Real(c3)] => second_case(c2, c3, result),
        [Char64::Sym,      Char64::Sym,      Char64::Real(c3)] => third_case(c3, result),
        [Char64::Real(c1), Char64::Sym,      Char64::Sym]      => fourth_case(c1, result),
        [Char64::Real(c1), Char64::Real(c2), Char64::Sym]      => fifth_case(c1, c2, result),
        _ => panic!()
    }
}

#[inline(always)]
fn first_case(c1: &u32, c2: &u32, c3: &u32, result: &mut Vec<u8>) {
    let group24: u32 = (c1 << 16) | (c2 << 8) | c3 ;
    result[0] = BASE64_CHARS[((group24 >> 18) & 0x3f) as usize];
    result[1] = BASE64_CHARS[((group24 >> 12) & 0x3f) as usize];
    result[2] = BASE64_CHARS[((group24 >> 6) & 0x3f) as usize];
    result[3] = BASE64_CHARS[(group24 & 0x3f) as usize];
}

#[inline(always)]
fn second_case(c2: &u32, c3: &u32, result: &mut Vec<u8>) {
    let group24: u32 = (0_u32 << 16) | (c2 << 8) | c3;
    result[0] = EQUALS;
    result[1] = BASE64_CHARS[((group24 >> 12) & 0x3f) as usize];
    result[2] = BASE64_CHARS[((group24 >> 6) & 0x3f) as usize];
    result[3] = BASE64_CHARS[(group24 & 0x3f) as usize];
}

#[inline(always)]
fn third_case(c3: &u32, result: &mut Vec<u8>) {
    let group24: u32 = (0_u32 << 16) | (0_u32 << 8) | c3;
    result[0] = EQUALS;
    result[1] = EQUALS;
    result[2] = BASE64_CHARS[((group24 >> 6) & 0x3f) as usize];
    result[3] = BASE64_CHARS[(group24 & 0x3f) as usize];
}

#[inline(always)]
fn fourth_case(c3: &u32, result: &mut Vec<u8>) {
    let group24: u32 = (c3 << 16) | (0_u32 << 8) | 0_u32;
    result[0] = BASE64_CHARS[((group24 >> 18) & 0x3f) as usize];
    result[1] = BASE64_CHARS[((group24 >> 12) & 0x3f) as usize];
    result[2] = EQUALS;
    result[3] = EQUALS;        
}

#[inline(always)]
fn fifth_case(c1: &u32, c2: &u32, result: &mut Vec<u8>) {
    let group24: u32 = (c1 << 16) | (c2 << 8) | 0_u32;
    result[0] = BASE64_CHARS[((group24 >> 18) & 0x3f) as usize];
    result[1] = BASE64_CHARS[((group24 >> 12) & 0x3f) as usize];
    result[2] = BASE64_CHARS[((group24 >> 6) & 0x3f) as usize];
    result[3] = EQUALS;        
}

#[test]
fn base64_three_chars_symbolic_first_case() {
    let array = [Char64::Real(0), Char64::Real(1), Char64::Real(3)];
    let mut output: Vec<u8> = vec![0, 0, 0, 0];
    base64_three_chars_symbolic(&array, &mut output);
    assert_eq!(output, vec![65, 65, 69, 68]);
}

#[test]
fn base64_three_chars_symbolic_second_case() {
    let array = [Char64::Sym, Char64::Real(1), Char64::Real(3)];
    let mut output: Vec<u8> = vec![0, 0, 0, 0];
    base64_three_chars_symbolic(&array, &mut output);
    assert_eq!(output, vec![61, 65, 69, 68]);
}

#[test]
fn base64_three_chars_symbolic_third_case() {
    let array = [Char64::Sym, Char64::Sym, Char64::Real(3)];
    let mut output: Vec<u8> = vec![0, 0, 0, 0];
    base64_three_chars_symbolic(&array, &mut output);
    assert_eq!(output, vec![61, 61, 65, 68]);
}

#[test]
fn base64_three_chars_symbolic_fourth_case() {
    let array = [Char64::Real(0), Char64::Sym, Char64::Sym];
    let mut output: Vec<u8> = vec![0, 0, 0, 0];
    base64_three_chars_symbolic(&array, &mut output);
    assert_eq!(output, vec![65, 65, 61, 61]);
}

#[test]
fn base64_three_chars_symbolic_fifth_case() {
    let array = [Char64::Real(0), Char64::Real(1), Char64::Sym];
    let mut output: Vec<u8> = vec![0, 0, 0, 0];
    base64_three_chars_symbolic(&array, &mut output);
    assert_eq!(output, vec![65, 65, 69, 61]);
}

