use regex::Regex;

pub const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
enum InChar64 {
    Sym,
    Real(u32)
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
enum OutChar64 {
    Equals,
    Single(u8),
    Multiple(Vec<u8>)
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
pub struct Candidates(Vec<OutChar64>, Vec<OutChar64>, Vec<OutChar64>);


impl From<&Candidates> for Regex {
    fn from(candidates: &Candidates) -> Self {
        Regex::new(&String::from(candidates)).unwrap()
    }
}

impl From<&OutChar64> for String {
    fn from(outchar: &OutChar64) -> Self {
        let mut out = String::new();
        match outchar {
            OutChar64::Single(byte) => out.push(*byte as char),
            OutChar64::Multiple(bytes) => {
                out.push('(');
                let multiple = bytes.
                                iter().
                                map(|b| (*b as char).to_string()).
                                collect::<Vec<String>>().join("|");
                out.push_str(&multiple);
                out.push(')');
            },
            //OutChar64::Equals => {out.push('=')}
            OutChar64::Equals => { }
        }
        out
    }
}


impl From<&Candidates> for String {
    fn from(candidates: &Candidates) -> Self {
        let mut out = String::new();
        out.push('(');
        let Candidates(first, second, third) = candidates;
        let str_candidates = vec![first, second, third].into_iter().map(|candidate| {
            candidate.iter().map(|outchar| String::from(outchar)).collect::<Vec<String>>().join("")
        }).collect::<Vec<String>>().join("|");
        out.push_str(&str_candidates);
        out.push(')');
        out
    }
}


pub fn generate_candidates(input: &String) -> Candidates {
    let mut input0: Vec<InChar64> = input.bytes().map(|b| { InChar64::Real(b as u32)}).collect();
    let mut input1: Vec<InChar64> = input0.clone();
    let mut input2: Vec<InChar64> = input0.clone();

    input1.insert(0, InChar64::Sym);
    input2.insert(0, InChar64::Sym); 
    input2.insert(0, InChar64::Sym);

    let mut out: Vec<OutChar64> = std::iter::repeat(OutChar64::Equals).take(4).collect();
    let outputs = vec![&mut input0, &mut input1, &mut input2].into_iter().map(|input| {
        let i = (3 - (input.len() % 3)) % 3;
        input.extend(std::iter::repeat(InChar64::Sym).take(i));
        input.
          chunks(3_usize).
          fold(Vec::new(), |mut accu, three_chars: &[InChar64]| {
                base64_three_chars_symbolic(three_chars, &mut out);
                accu.append(&mut out.clone());
                accu
            })
    }).collect::<Vec<Vec<OutChar64>>>();
    Candidates(outputs[0].clone(), outputs[1].clone(), outputs[2].clone())
}

#[test]
fn test_generate_candidates_regex_empty() {
    assert_eq!("(||)", String::from(&generate_candidates(&String::from(""))));
}

#[test]
fn test_generate_candidates_regex_simple_a() {
    let first = "Q(Q|R|S|T|U|V|W|X|Y|Z|a|b|c|d|e|f)";
    let second = "(E|U|k|0)(E|F|G|H)";
    let third = "(B|F|J|N|R|V|Z|d|h|l|p|t|x|1|5|9)B";
    let a_result = format!("({}|{}|{})", first, second, third);
    assert_eq!(a_result, String::from(&generate_candidates(&String::from("A"))));
}

#[test]
fn test_generate_candidates_sanity_checks() {
    assert_ne!(generate_candidates(&String::from("A")), generate_candidates(&String::from("B")));
    assert_eq!(generate_candidates(&String::from("A")), generate_candidates(&String::from("A")));
}


// ASCII-Art as PNG: 
//
//   https://web.archive.org/web/20210409165509/https://upload.wikimedia.org/wikipedia/commons/7/70/Base64-de.png
//
// +-------------------------------------------+
// |                                           |
// |            I        II        III         |
// | ascii |876543_21|8765_4321|87_654321|     |
// | b64   |654321|65_4321|6543_21|654321|     |
// |          a       b       c      d         |
// |                                           |
// +-------------------------------------------+
//
//  Five cases exist:
//
//  1)  I, II and III are given   -> full knowledge of {a, b, c ,d}  partially: {}
//  2)  II and III are given      -> full knowledge of {c, d}        partially: {b}
//  3)  Only III is given         -> full knowledge of {d}           partially: {c}
//  4)  Only I is given           -> full knowledge of {a}           partially: {b}
//  5)  I and II are given        -> full knowledge of {a, b}        partially: {c}
//  6)  Only II is given          -> full knowledge of {}            partially: {b,c}
//  7)  None is given             -> full knowledge of {}            partially: {}
fn base64_three_chars_symbolic(chars: &[InChar64], result: &mut Vec<OutChar64>) {
    match chars {
        [InChar64::Real(c1), InChar64::Real(c2), InChar64::Real(c3)] => first_case(c1, c2, c3, result),
        [InChar64::Sym,      InChar64::Real(c2), InChar64::Real(c3)] => second_case(c2, c3, result),
        [InChar64::Sym,      InChar64::Sym,      InChar64::Real(c3)] => third_case(c3, result),
        [InChar64::Real(c1), InChar64::Sym,      InChar64::Sym]      => fourth_case(c1, result),
        [InChar64::Real(c1), InChar64::Real(c2), InChar64::Sym]      => fifth_case(c1, c2, result),
        [InChar64::Sym,      InChar64::Real(c2), InChar64::Sym]      => sixth_case(c2, result),
        [InChar64::Sym,      InChar64::Sym,      InChar64::Sym]      => seventh_case(result),
        _ => panic!("Can not handle this case (length: {}): {:?}", chars.len(), chars)
    }
}

#[inline(always)]
fn first_case(c1: &u32, c2: &u32, c3: &u32, result: &mut Vec<OutChar64>) {
    let group24: u32 = (c1 << 16) | (c2 << 8) | c3 ;
    result[0] = OutChar64::Single(BASE64_CHARS[((group24 >> 18) & 0x3f) as usize]);
    result[1] = OutChar64::Single(BASE64_CHARS[((group24 >> 12) & 0x3f) as usize]);
    result[2] = OutChar64::Single(BASE64_CHARS[((group24 >> 6) & 0x3f) as usize]);
    result[3] = OutChar64::Single(BASE64_CHARS[(group24 & 0x3f) as usize]);
}

// +-------------------------------------------+
// |            I        II        III         |
// | ascii |SSSSSS_SS|8765_4321|87_654321|     |
// | b64   |XXXXXX|??_4321|6543_21|654321|     |
// |          a       b       c      d         |
// +-------------------------------------------+
#[inline(always)]
fn second_case(c2: &u32, c3: &u32, result: &mut Vec<OutChar64>) {
    let group24: u32 = (0_u32 << 16) | (c2 << 8) | c3;
    result[0] = OutChar64::Equals;
    let partial = ((group24 >> 12) & 0b001111) as usize; 
    result[1] = OutChar64::Multiple((0..4).map(|n| BASE64_CHARS[partial + (n * 16)]).collect::<Vec<u8>>()); // mult
    result[2] = OutChar64::Single(BASE64_CHARS[((group24 >> 6) & 0x3f) as usize]);
    result[3] = OutChar64::Single(BASE64_CHARS[(group24 & 0x3f) as usize]);
}

// +-------------------------------------------+
// |            I        II        III         |
// | ascii |SSSSSS_SS|SSSS_SSSS|87_654321|     |
// | b64   |XXXXXX|XX_XXXX|????_21|654321|     |
// |          a       b       c      d         |
// +-------------------------------------------+
//#[inline(always)]
fn third_case(c3: &u32, result: &mut Vec<OutChar64>) {
    let group24: u32 = (0_u32 << 16) | (0_u32 << 8) | c3;
    result[0] = OutChar64::Equals;
    result[1] = OutChar64::Equals;
    let partial = ((group24 >> 6) & 0b000011) as usize; 
    result[2] = OutChar64::Multiple((0..16).map(|n| { BASE64_CHARS[partial + (n * 4)]}).collect::<Vec<u8>>()); // mult
    result[3] = OutChar64::Single(BASE64_CHARS[(group24 & 0x3f) as usize]);
}

// +-------------------------------------------+
// |            I        II        III         |
// | ascii |876543_21|SSSS_SSSS|SS_SSSSSS|     |
// | b64   |654321|65_????|XXXX_XX|XXXXXX|     |
// |          a       b       c      d         |
// +-------------------------------------------+
#[inline(always)]
fn fourth_case(c3: &u32, result: &mut Vec<OutChar64>) {
    let group24: u32 = (c3 << 16) | (0_u32 << 8) | 0_u32;
    result[0] = OutChar64::Single(BASE64_CHARS[((group24 >> 18) & 0x3f) as usize]);
    let partial = ((group24 >> 12) & 0b110000) as usize; 
    result[1] = OutChar64::Multiple((0..16).map(|n| BASE64_CHARS[partial | n]).collect::<Vec<u8>>()); // mult
    result[2] = OutChar64::Equals;
    result[3] = OutChar64::Equals;        
}

// +-------------------------------------------+
// |            I        II        III         |
// | ascii |876543_21|8765_4321|SS_SSSSSS|     |
// | b64   |654321|65_4321|6543_??|XXXXXX|     |
// |          a       b       c      d         |
// +-------------------------------------------+
#[inline(always)]
fn fifth_case(c1: &u32, c2: &u32, result: &mut Vec<OutChar64>) {
    let group24: u32 = (c1 << 16) | (c2 << 8) | 0_u32;
    result[0] = OutChar64::Single(BASE64_CHARS[((group24 >> 18) & 0x3f) as usize]);
    result[1] = OutChar64::Single(BASE64_CHARS[((group24 >> 12) & 0x3f) as usize]);
    let partial = ((group24 >> 6) & 0b111100) as usize; 
    result[2] = OutChar64::Multiple((0..4).map(|n| BASE64_CHARS[partial | n]).collect::<Vec<u8>>()); // mult
    result[3] = OutChar64::Equals;        
}

// +-------------------------------------------+
// |            I        II        III         |
// | ascii |SSSSSS_SS|8765_4321|SS_SSSSSS|     |
// | b64   |XXXXXX|??_4321|6543_??|XXXXXX|     |
// |          a       b       c      d         |
// +-------------------------------------------+
#[inline(always)]
fn sixth_case(c2: &u32, result: &mut Vec<OutChar64>) {
    let group24: u32 = (0_u32 << 16) | (c2 << 8) | 0_u32;
    result[0] = OutChar64::Equals;        
    let partial = ((group24 >> 12) & 0b001111) as usize; 
    result[1] = OutChar64::Multiple((0..4).map(|n| BASE64_CHARS[partial + (n * 16)]).collect::<Vec<u8>>()); // mult
    let partial = ((group24 >> 6) & 0b111100) as usize; 
    result[2] = OutChar64::Multiple((0..4).map(|n| BASE64_CHARS[partial | n]).collect::<Vec<u8>>()); // mult
    result[3] = OutChar64::Equals;        
}

// +-------------------------------------------+
// |            I        II        III         |
// | ascii |SSSSSS_SS|SSSS_SSSS|SS_SSSSSS|     |
// | b64   |XXXXXX|XX_XXXX|XXXX_XX|XXXXXX|     |
// |          a       b       c      d         |
// +-------------------------------------------+
#[inline(always)]
fn seventh_case(result: &mut Vec<OutChar64>) {
    result[0] = OutChar64::Equals;        
    result[1] = OutChar64::Equals;
    result[2] = OutChar64::Equals;
    result[3] = OutChar64::Equals;        
}


#[test]
fn base64_three_chars_symbolic_first_case() {
    let array = [InChar64::Real(0), InChar64::Real(1), InChar64::Real(3)];
    let mut output: Vec<OutChar64> = vec![OutChar64::Equals, OutChar64::Equals, OutChar64::Equals, OutChar64::Equals];
    base64_three_chars_symbolic(&array, &mut output);
    assert_eq!(output, vec![OutChar64::Single(65), OutChar64::Single(65), OutChar64::Single(69), OutChar64::Single(68)]);
}

#[test]
fn base64_three_chars_symbolic_second_case() {
    let array = [InChar64::Sym, InChar64::Real(1), InChar64::Real(3)];
    let mut output: Vec<OutChar64> = vec![OutChar64::Equals, OutChar64::Equals, OutChar64::Equals, OutChar64::Equals];
    base64_three_chars_symbolic(&array, &mut output);
    assert_eq!(output, vec![OutChar64::Equals, OutChar64::Multiple(vec![65, 81, 103, 119]), OutChar64::Single(69), OutChar64::Single(68)]);
}

#[test]
fn base64_three_chars_symbolic_third_case() {
    let array = [InChar64::Sym, InChar64::Sym, InChar64::Real(3)];
    let mut output: Vec<OutChar64> = vec![OutChar64::Equals, OutChar64::Equals, OutChar64::Equals, OutChar64::Equals];
    base64_three_chars_symbolic(&array, &mut output);
    assert_eq!(output, vec![OutChar64::Equals, OutChar64::Equals, OutChar64::Multiple(vec![65, 69, 73, 77, 81, 85, 89, 99, 103, 107, 111, 115, 119, 48, 52, 56]), OutChar64::Single(68)]);
}

#[test]
fn base64_three_chars_symbolic_fourth_case() {
    let array = [InChar64::Real(0), InChar64::Sym, InChar64::Sym];
    let mut output: Vec<OutChar64> = vec![OutChar64::Equals, OutChar64::Equals, OutChar64::Equals, OutChar64::Equals];
    base64_three_chars_symbolic(&array, &mut output);
    assert_eq!(output, vec![OutChar64::Single(65), OutChar64::Multiple(vec![65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80]), OutChar64::Equals, OutChar64::Equals]);
}

#[test]
fn base64_three_chars_symbolic_fifth_case() {
    let array = [InChar64::Real(0), InChar64::Real(1), InChar64::Sym];
    let mut output: Vec<OutChar64> = vec![OutChar64::Equals, OutChar64::Equals, OutChar64::Equals, OutChar64::Equals];
    base64_three_chars_symbolic(&array, &mut output);
    assert_eq!(output, vec![OutChar64::Single(65), OutChar64::Single(65), OutChar64::Multiple(vec![69, 70, 71, 72]), OutChar64::Equals]);
}

