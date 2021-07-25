mod playgroud;

use std::env;
use std::io;
//use std::io::{Write,Read};
use std::io::Read;
//use std::io::Bytes;

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
fn base64_encode_symbolic(input: &String) -> Vec<String> {
    let mut input0: String = input.clone();
    let mut input1: String = input.clone();
    let mut input2: String = input.clone();

    input1.insert(0, '\0');
    input2.insert_str(0, "\0\0");

    let mut out: Vec<u8> = vec![0, 0, 0, 0];

    let mut outputs = vec![&mut input0, &mut input1, &mut input2].into_iter().map(|input| {
        let i = (3 - (input.len() % 3)) % 3;
        input.extend(std::iter::repeat('\0').take(i));

        let mut cont = input.bytes().
            collect::<Vec<u8>>().
            chunks(3_usize).
            map(|three_chars: &[u8]| {
                base64_three_chars(three_chars, &mut out);
                out.iter().map(|&x| x as char).collect::<String>()
            }).collect::<Vec<String>>().join("");

        let len = cont.len();
        cont.replace_range(len-i..len, &String::from("=").repeat(i));
        cont
    }).collect::<Vec<String>>();
    outputs[1].replace_range(0..1, &String::from("=").repeat(1));
    outputs[2].replace_range(0..2, &String::from("=").repeat(2));
    outputs
}

//#[test]
//fn test_base64_symbolic_empty() {
//    //let empty: std::io::Bytes<&[u8]> = vec!(b"".bytes(), b"=A==".bytes(), b"==A=".bytes());
//    //let empty: Vec<Bytes> = vec!(b"".bytes(), b"=A==".bytes(), b"==A=".bytes());
//    let empty: Vec<&[u8]> = vec!(b"", b"=a==", b"==a=");
//    assert_eq!(base64_encode_symbolic(b"".bytes()), empty);
//}
//
//#[test]
//fn test_base64_symbolic_a() {
//    let empty = vec!(b"YQ==", b"=GE=", b"==Bh");
//    assert_eq!(base64_encode_symbolic(b"a".bytes()), empty);
//}
//
//#[test]
//fn test_base64_symbolic_za() {
//    let empty = vec!(b"emE=", b"=Hph", b"==B6YQ==");
//    assert_eq!(base64_encode_symbolic(b"za".bytes()), empty);
//}
//
//#[test]
//fn test_base64_symbolic_az() {
//    let empty = vec!(b"YXo=", b"=GF6", b"==Bheg==");
//    assert_eq!(base64_encode_symbolic(b"az".bytes()), empty);
//}
//
//#[test]
//fn test_base64_symbolic_zzz() {
//    let empty = vec!(b"enp6", b"=Hp6eg==", b"==B6eno=");
//    assert_eq!(base64_encode_symbolic(b"zzz".bytes()), empty);
//}
//
//#[test]
//fn test_base64_symbolic_zzzz() {
//    let empty = vec!(b"enp6eg==", b"=Hp6eno=", b"==B6enp6");
//    assert_eq!(base64_encode_symbolic(b"zzzz".bytes()), empty);
//}
//
//#[test]
//fn test_base64_symbolic_change_me_zzzz() {
//    let empty = vec!(b"enp6eg==", b"=Hp6eno=", b"==B6enp6");
//    assert_eq!(base64_encode_symbolic(b"zzzz".bytes()), empty);
//    panic!();
//}

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
