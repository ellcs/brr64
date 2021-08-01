mod playgroud;

use std::env;
use std::io;
//use std::io::{Write,Read};
use std::io::Read;
use std::str::Bytes;

const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
    let mut input = String::new();
    let _args: Vec<String> = env::args().collect();

    while let Ok(_n) = io::stdin().read_line(&mut input) {
        // rm -f \n
        input.pop();
        let output = base64_encode(&input);
        println!("{}", output);
        input.clear();
    }
}

fn base64_encode(input: &String) -> String {
    let mut output = input.clone();
    let mut out: Vec<u8> = vec![0, 0, 0, 0];

    let i = (3 - (output.len() % 3)) % 3;
    output.extend(std::iter::repeat('\0').take(i));

    output = output.bytes().
        collect::<Vec<u8>>().
        chunks(3_usize).
        map(|three_chars: &[u8]| {
            base64_three_chars(three_chars, &mut out);
            out.iter().map(|&x| x as char).collect::<String>()
        }).collect::<Vec<String>>().join("");

    let len = output.len();
    output.replace_range(len-i..len, &String::from("=").repeat(i));
    output
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
                base64_three_chars(three_chars, &mut out);
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

#[test]
fn test_base64_encode_aa() {
    assert_eq!(base64_encode(&String::from("aa")), "YWE=");
}

#[test]
fn test_base64_encode_a() {
    assert_eq!(base64_encode(&String::from("a")), "YQ==");
}

#[test]
fn test_base64_encode_asdqw() {
    assert_eq!(base64_encode(&String::from("asdqw")), "YXNkcXc=");
}


#[inline(always)]
fn base64_three_chars(chars: &[u8], result: &mut Vec<u8>) {
    let first_byte: u32 = chars[0] as u32;
    let second_byte: u32 = chars[1] as u32;
    let third_byte: u32 = chars[2] as u32;
    let group24: u32 =  (first_byte << 16) | (second_byte << 8) | third_byte;
    result[0] = BASE64_CHARS[((group24 >> 18) & 0x3f) as usize];
    result[1] = BASE64_CHARS[((group24 >> 12) & 0x3f) as usize];
    result[2] = BASE64_CHARS[((group24 >> 6) & 0x3f) as usize];
    result[3] = BASE64_CHARS[(group24 & 0x3f) as usize];
}

#[test]
fn test_base64_three_chars_simple_aaa() {
    let mut result: Vec<u8> = vec![0, 0, 0, 0];
    let input: Vec<u8> = "AAA".bytes().collect::<Vec<u8>>();
    base64_three_chars(&input, &mut result);
    assert_eq!(&result, b"QUFB");
}

#[test]
#[should_panic]
fn test_base64_three_chars_not_enough_chars_ab() {
    let mut result: Vec<u8> = vec![0, 0, 0, 0];
    let input: Vec<u8> = "ab".bytes().collect::<Vec<u8>>();
    base64_three_chars(&input, &mut result);
    assert_eq!(&result, b"YWJj");
}

#[test]
fn test_base64_three_chars_simple_abc() {
    let mut result: Vec<u8> = vec![0, 0, 0, 0];
    let input: Vec<u8> = "abc".bytes().collect::<Vec<u8>>();
    base64_three_chars(&input, &mut result);
    assert_eq!(&result, b"YWJj");
}
