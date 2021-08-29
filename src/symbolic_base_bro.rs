/// Classical base64 characters. You can find there everywhere on the internet.
pub const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

 
/// Representation of input characters for brr64
///
/// It's either symbolic or real. You can think of the classical padding, when reading symbolic.
/// But it's not aquivalent. :)
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
pub enum InChar64 {
    Sym,
    Real(u32)
}


/// Representation of output characters for brr64
///
/// You may know the classical equal sign `=` from base64. It occurs when padding is added at the
/// end. This is one case. The other option is, that you have a classical character, which is the
/// `Single` case. The `Multiple` option is not known to base64. This happens when the source-byte
/// was symbolic and not all informations were provided for this character. In that case all
/// possible cases are provided within the `Multiple` case.
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
pub enum OutChar64 {
    Equals,
    Single(u8),
    Multiple(Vec<u8>)
}


/// brr64's internal representation of the 'plain-text' you're looking for.
///
/// There are three possible candidates for the text you're looking for:
///   - The first candidate is the normal base64 case, with the padding known from base64. But
///     instead of padding `\0`-bytes, we pad with the `InChar64::Sym` as symbolic value.
///   - The second candidate is the result of base64, that had a symbolic byte (`InChar64::Sym`)
///     prepended and than been padded, similar to the first case.
///   - The thrid candidate is similar to the second, but has two symbolic bytes prepended and the
///     the symbolic padding.
///
/// All padding at the end of the candidates is optional, as usual in base64.
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
pub struct Candidates(pub Vec<OutChar64>, pub Vec<OutChar64>, pub Vec<OutChar64>);


pub fn generate_candidates(input: &str) -> Candidates {
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


/// Matches given three InChars64 (Symbolic or Real) to various cases and provides the pseudo
/// base64 output for them.
///
/// There are seven cases documented for the three provided input characters.
///   1.  I, II and III are given   -> full knowledge of {a, b, c ,d}  partially: {}
///   2.  II and III are given      -> full knowledge of {c, d}        partially: {b}
///   3.  Only III is given         -> full knowledge of {d}           partially: {c}
///   4.  Only I is given           -> full knowledge of {a}           partially: {b}
///   5.  I and II are given        -> full knowledge of {a, b}        partially: {c}
///   6.  Only II is given          -> full knowledge of {}            partially: {b,c}
///   7.  None is given             -> full knowledge of {}            partially: {}
///
/// Match the given roman numerals (I, II, III) and alphabetical characters (a,b,c,d):
///
///  +-------------------------------------------+
///  |                                           |
///  |            I        II        III         |
///  | ascii |876543_21|8765_4321|87_654321|     |
///  | b64   |654321|65_4321|6543_21|654321|     |
///  |          a       b       c      d         |
///  |                                           |
///  +-------------------------------------------+
///
///   https://web.archive.org/web/20210409165509/https://upload.wikimedia.org/wikipedia/commons/7/70/Base64-de.png
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


/// +-------------------------------------------+
/// |            I        II        III         |
/// | ascii |SSSSSS_SS|8765_4321|87_654321|     |
/// | b64   |XXXXXX|??_4321|6543_21|654321|     |
/// |          a       b       c      d         |
/// +-------------------------------------------+
/// S - symbolic
/// X - not needed
/// ? - unknown; but partial information exists and we have to consider all possible options.
#[inline(always)]
fn second_case(c2: &u32, c3: &u32, result: &mut Vec<OutChar64>) {
    // (0_u32 << 16) | (c2 << 8) | c3;
    let group24: u32 = (c2 << 8) | c3;
    result[0] = OutChar64::Equals;
    let partial = ((group24 >> 12) & 0b001111) as usize; 
    result[1] = OutChar64::Multiple((0..4).map(|n| BASE64_CHARS[partial + (n * 16)]).collect::<Vec<u8>>()); // mult
    result[2] = OutChar64::Single(BASE64_CHARS[((group24 >> 6) & 0x3f) as usize]);
    result[3] = OutChar64::Single(BASE64_CHARS[(group24 & 0x3f) as usize]);
}


/// +-------------------------------------------+
/// |            I        II        III         |
/// | ascii |SSSSSS_SS|SSSS_SSSS|87_654321|     |
/// | b64   |XXXXXX|XX_XXXX|????_21|654321|     |
/// |          a       b       c      d         |
/// +-------------------------------------------+
/// S - symbolic
/// X - not needed
/// ? - unknown; but partial information exists and we have to consider all possible options.
#[inline(always)]
fn third_case(c3: &u32, result: &mut Vec<OutChar64>) {
    // (0_u32 << 16) | (0_u32 << 8) | c3;
    let group24 = c3;
    result[0] = OutChar64::Equals;
    result[1] = OutChar64::Equals;
    let partial = ((group24 >> 6) & 0b000011) as usize; 
    result[2] = OutChar64::Multiple((0..16).map(|n| { BASE64_CHARS[partial + (n * 4)]}).collect::<Vec<u8>>()); // mult
    result[3] = OutChar64::Single(BASE64_CHARS[(group24 & 0x3f) as usize]);
}


/// +-------------------------------------------+
/// |            I        II        III         |
/// | ascii |876543_21|SSSS_SSSS|SS_SSSSSS|     |
/// | b64   |654321|65_????|XXXX_XX|XXXXXX|     |
/// |          a       b       c      d         |
/// +-------------------------------------------+
/// S - symbolic
/// X - not needed
/// ? - unknown; but partial information exists and we have to consider all possible options.
#[inline(always)]
fn fourth_case(c3: &u32, result: &mut Vec<OutChar64>) {
    // (c3 << 16) | (0_u32 << 8) | 0_u32;
    let group24: u32 = c3 << 16;
    result[0] = OutChar64::Single(BASE64_CHARS[((group24 >> 18) & 0x3f) as usize]);
    let partial = ((group24 >> 12) & 0b110000) as usize; 
    result[1] = OutChar64::Multiple((0..16).map(|n| BASE64_CHARS[partial | n]).collect::<Vec<u8>>()); // mult
    result[2] = OutChar64::Equals;
    result[3] = OutChar64::Equals;        
}


/// +-------------------------------------------+
/// |            I        II        III         |
/// | ascii |876543_21|8765_4321|SS_SSSSSS|     |
/// | b64   |654321|65_4321|6543_??|XXXXXX|     |
/// |          a       b       c      d         |
/// +-------------------------------------------+
/// S - symbolic
/// X - not needed
/// ? - unknown; but partial information exists and we have to consider all possible options.
#[inline(always)]
fn fifth_case(c1: &u32, c2: &u32, result: &mut Vec<OutChar64>) {
    // (c1 << 16) | (c2 << 8) | 0_u32;
    let group24: u32 = (c1 << 16) | (c2 << 8);
    result[0] = OutChar64::Single(BASE64_CHARS[((group24 >> 18) & 0x3f) as usize]);
    result[1] = OutChar64::Single(BASE64_CHARS[((group24 >> 12) & 0x3f) as usize]);
    let partial = ((group24 >> 6) & 0b111100) as usize; 
    result[2] = OutChar64::Multiple((0..4).map(|n| BASE64_CHARS[partial | n]).collect::<Vec<u8>>()); // mult
    result[3] = OutChar64::Equals;        
}


/// +-------------------------------------------+
/// |            I        II        III         |
/// | ascii |SSSSSS_SS|8765_4321|SS_SSSSSS|     |
/// | b64   |XXXXXX|??_4321|6543_??|XXXXXX|     |
/// |          a       b       c      d         |
/// +-------------------------------------------+
/// S - symbolic
/// X - not needed
/// ? - unknown; but partial information exists and we have to consider all possible options.
#[inline(always)]
fn sixth_case(c2: &u32, result: &mut Vec<OutChar64>) {
    // (0_u32 << 16) | (c2 << 8) | 0_u32;
     let group24: u32 = c2 << 8;
    result[0] = OutChar64::Equals;        
    let partial = ((group24 >> 12) & 0b001111) as usize; 
    result[1] = OutChar64::Multiple((0..4).map(|n| BASE64_CHARS[partial + (n * 16)]).collect::<Vec<u8>>()); // mult
    let partial = ((group24 >> 6) & 0b111100) as usize; 
    result[2] = OutChar64::Multiple((0..4).map(|n| BASE64_CHARS[partial | n]).collect::<Vec<u8>>()); // mult
    result[3] = OutChar64::Equals;        
}


/// +-------------------------------------------+
/// |            I        II        III         |
/// | ascii |SSSSSS_SS|SSSS_SSSS|SS_SSSSSS|     |
/// | b64   |XXXXXX|XX_XXXX|XXXX_XX|XXXXXX|     |
/// |          a       b       c      d         |
/// +-------------------------------------------+
/// S - Symbolic
/// X - Not needed
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
    let expected = vec![OutChar64::Single(65), 
                        OutChar64::Single(65), 
                        OutChar64::Single(69), 
                        OutChar64::Single(68)];
    assert_eq!(output, expected);
}

#[test]
fn base64_three_chars_symbolic_second_case() {
    let array = [InChar64::Sym, InChar64::Real(1), InChar64::Real(3)];
    let mut output: Vec<OutChar64> = vec![OutChar64::Equals, OutChar64::Equals, OutChar64::Equals, OutChar64::Equals];
    base64_three_chars_symbolic(&array, &mut output);
    let expected = vec![OutChar64::Equals, 
                        OutChar64::Multiple(vec![65, 81, 103, 119]), 
                        OutChar64::Single(69), 
                        OutChar64::Single(68)];
    assert_eq!(output, expected);
}

#[test]
fn base64_three_chars_symbolic_third_case() {
    let array = [InChar64::Sym, InChar64::Sym, InChar64::Real(3)];
    let mut output: Vec<OutChar64> = vec![OutChar64::Equals, OutChar64::Equals, OutChar64::Equals, OutChar64::Equals];
    base64_three_chars_symbolic(&array, &mut output);
    let expected = vec![OutChar64::Equals, 
                        OutChar64::Equals, 
                        OutChar64::Multiple(vec![65, 69, 73, 77, 81, 85, 89, 99, 103, 107, 111, 115, 119, 48, 52, 56]), 
                        OutChar64::Single(68)];
    assert_eq!(output, expected);
}

#[test]
fn base64_three_chars_symbolic_fourth_case() {
    let array = [InChar64::Real(0), InChar64::Sym, InChar64::Sym];
    let mut output: Vec<OutChar64> = vec![OutChar64::Equals, OutChar64::Equals, OutChar64::Equals, OutChar64::Equals];
    base64_three_chars_symbolic(&array, &mut output);
    let expected = vec![OutChar64::Single(65), 
                        OutChar64::Multiple(vec![65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80]), 
                        OutChar64::Equals, 
                        OutChar64::Equals];
    assert_eq!(output, expected);
}

#[test]
fn base64_three_chars_symbolic_fifth_case() {
    let array = [InChar64::Real(0), InChar64::Real(1), InChar64::Sym];
    let mut output: Vec<OutChar64> = vec![OutChar64::Equals, OutChar64::Equals, OutChar64::Equals, OutChar64::Equals];
    base64_three_chars_symbolic(&array, &mut output);
    let expected = vec![OutChar64::Single(65), 
                        OutChar64::Single(65), 
                        OutChar64::Multiple(vec![69, 70, 71, 72]), 
                        OutChar64::Equals];
    assert_eq!(output, expected);
}

