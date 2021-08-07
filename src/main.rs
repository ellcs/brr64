mod playgroud;
mod base64_encode;

use std::env;
use std::io;


fn main() {
    let mut input = String::new();
    let _args: Vec<String> = env::args().collect();

    while let Ok(_n) = io::stdin().read_line(&mut input) {
        // rm -f \n
        input.pop();
        let output = base64_encode::base64_encode(&input);
        println!("{}", output);
        input.clear();
    }
}

//fn base64_encode_symbolic(input: &String) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
//fn base64_encode_symbolic(input: &Bytes) -> Bytes {
fn base64_encode_symbolic(input: Vec<u8>) -> Vec<Vec<u8>> {
    let mut input0: Vec<u8> = input.clone();
    let mut input1: Vec<u8> = input.clone();
    let mut input2: Vec<u8> = input.clone();

    input1.insert(0, b'\0');
    input2.insert(0, b'\0'); input2.insert(0, b'\0');

    let mut out: Vec<u8> = vec![0, 0, 0, 0];

    let mut outputs = vec![&mut input0, &mut input1, &mut input2].into_iter().map(|input| {
        let i = (3 - (input.len() % 3)) % 3;
        input.extend(std::iter::repeat(b'\0').take(i));

        let mut cont: Vec<u8> = input.
            chunks(3_usize).
            fold(Vec::new(), |mut accu, three_chars: &[u8]| {
                base64_encode::base64_three_chars(three_chars, &mut out);
                accu.append(&mut out.clone());
                accu
            });

        let len = cont.len();
        cont.splice(len-i..len, std::iter::repeat(b'=').take(i).collect::<Vec<u8>>());
        cont
    }).collect::<Vec<Vec<u8>>>();
    outputs[1].splice(0..1, std::iter::repeat(b'=').take(1).collect::<Vec<u8>>());
    outputs[2].splice(0..2, std::iter::repeat(b'=').take(2).collect::<Vec<u8>>());
    outputs
}

enum Char64 {
    Sym,
    Real(u32)
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
    result[0] = base64_encode::BASE64_CHARS[((group24 >> 18) & 0x3f) as usize];
    result[1] = base64_encode::BASE64_CHARS[((group24 >> 12) & 0x3f) as usize];
    result[2] = base64_encode::BASE64_CHARS[((group24 >> 6) & 0x3f) as usize];
    result[3] = base64_encode::BASE64_CHARS[(group24 & 0x3f) as usize];
}

#[inline(always)]
fn second_case(c2: &u32, c3: &u32, result: &mut Vec<u8>) {
    let group24: u32 = (0_u32 << 16) | (c2 << 8) | c3;
    result[0] = base64_encode::EQUALS;
    result[1] = base64_encode::BASE64_CHARS[((group24 >> 12) & 0x3f) as usize];
    result[2] = base64_encode::BASE64_CHARS[((group24 >> 6) & 0x3f) as usize];
    result[3] = base64_encode::BASE64_CHARS[(group24 & 0x3f) as usize];
}

#[inline(always)]
fn third_case(c3: &u32, result: &mut Vec<u8>) {
    let group24: u32 = (0_u32 << 16) | (0_u32 << 8) | c3;
    result[0] = base64_encode::EQUALS;
    result[1] = base64_encode::EQUALS;
    result[2] = base64_encode::BASE64_CHARS[((group24 >> 6) & 0x3f) as usize];
    result[3] = base64_encode::BASE64_CHARS[(group24 & 0x3f) as usize];
}

#[inline(always)]
fn fourth_case(c3: &u32, result: &mut Vec<u8>) {
    let group24: u32 = (c3 << 16) | (0_u32 << 8) | 0_u32;
    result[0] = base64_encode::BASE64_CHARS[((group24 >> 18) & 0x3f) as usize];
    result[1] = base64_encode::BASE64_CHARS[((group24 >> 12) & 0x3f) as usize];
    result[2] = base64_encode::EQUALS;
    result[3] = base64_encode::EQUALS;        
}

#[inline(always)]
fn fifth_case(c1: &u32, c2: &u32, result: &mut Vec<u8>) {
    let group24: u32 = (c1 << 16) | (c2 << 8) | 0_u32;
    result[0] = base64_encode::BASE64_CHARS[((group24 >> 18) & 0x3f) as usize];
    result[1] = base64_encode::BASE64_CHARS[((group24 >> 12) & 0x3f) as usize];
    result[2] = base64_encode::BASE64_CHARS[((group24 >> 6) & 0x3f) as usize];
    result[3] = base64_encode::EQUALS;        
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

fn symbolic_test(input: &[u8], vecs: Vec<&[u8]>) {
    let empty: Vec<Vec<u8>> = vecs.iter().map(|b| b.to_vec()).collect();
    assert_eq!(base64_encode_symbolic(input.to_vec()), empty);
}

#[test]
fn test_base64_symbolic_empty() {
    symbolic_test(b"", vec!(b"", b"=A==", b"==A="));
}

#[test]
fn test_base64_symbolic_a() {
    let empty = vec!(b"YQ==".to_vec(), b"=GE=".to_vec(), b"==Bh".to_vec());
    assert_eq!(base64_encode_symbolic(b"a".to_vec()), empty);
}

#[test]
fn test_base64_symbolic_za() {
    let empty = vec!(b"emE=".to_vec(), b"=Hph".to_vec(), b"==B6YQ==".to_vec());
    assert_eq!(base64_encode_symbolic(b"za".to_vec()), empty);
}

#[test]
fn test_base64_symbolic_az() {
    let empty = vec!(b"YXo=".to_vec(), b"=GF6".to_vec(), b"==Bheg==".to_vec());
    assert_eq!(base64_encode_symbolic(b"az".to_vec()), empty);
}

#[test]
fn test_base64_symbolic_zzz() {
    let empty = vec!(b"enp6".to_vec(), b"=Hp6eg==".to_vec(), b"==B6eno=".to_vec());
    assert_eq!(base64_encode_symbolic(b"zzz".to_vec()), empty);
}

#[test]
fn test_base64_symbolic_zzzz() {
    let empty = vec!(b"enp6eg==".to_vec(), b"=Hp6eno=".to_vec(), b"==B6enp6".to_vec());
    assert_eq!(base64_encode_symbolic(b"zzzz".to_vec()), empty);
}

#[test]
fn test_base64_symbolic_change_me_zzzz() {
    let empty = vec!(b"enp6eg==".to_vec(), b"=Hp6eno=".to_vec(), b"==B6enp6".to_vec());
    assert_eq!(base64_encode_symbolic(b"zzzz".to_vec()), empty);
}

