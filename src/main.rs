use std::io;
use std::io::Write;

const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
	let mut input = String::new();

	while true {
		match io::stdin().read_line(&mut input) {
		  Ok(n) => {
            // rm -f \n
            input.pop();
            base64_encode(input.clone());
            input.clear();
		  }
		  Err(error) => println!("error: {}", error),
		}
	}
}


fn base64_encode(input: String) -> String {
    let mut output = input.clone();
    let mut out: Vec<u8> = vec![0, 0, 0, 0];

    let mut i = 0;
    while output.len() % 3 != 0 {
        output.push('\0');
        i = i + 1; 
    }

    output = output.bytes().
        collect::<Vec<u8>>().
        chunks(3 as usize).
        map(|three_chars: &[u8]| {
            base64_three_chars(&three_chars, &mut out);
            out.iter().map(|&x| x as char).collect::<String>()
        }).collect::<Vec<String>>().join("");
    //.flatten().collect::<String>();
    println!("Overwriteing {} chars with eq", i);
    let len = output.len();
    output.replace_range(len-i..len, &String::from("=").repeat(i));
    println!("{}", output);
    output
}


fn every_three_chars<F: FnMut(&[u8])>(s: &str, steps: u8, mut f: F) -> () {
    s.bytes().
        collect::<Vec<u8>>().
        chunks(steps as usize).
        for_each(|c| {
            f(&c)
        });
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

fn fooadd(a: &u8, b: &u8) -> u8 {
   a + b 
}


#[test]
#[should_panic]
fn test_overflows() {
    let a: u8 = 255;
    let b: u8 = 1;
    fooadd(&a, &b);
}

#[test]
fn test_make_string_from_byte_vec() {
	let bytevec: Vec<u8> = vec![0x41, 0x42, 0x43];
	//let s: String = bytevec.collect();
}

#[test]
fn test_chunks() {
    let s: &str = "asdf asdf";
    s.bytes().
        collect::<Vec<u8>>().
        chunks(2 as usize).
        for_each(|c| {
            let _a: u8 = c.iter().sum();
        })
}


#[test]
fn test_rev_str() {
    let s: &str = "asdf";
    let r: String = s.chars().rev().collect();
    assert!(&r == "fdsa");
}


#[test]
fn test_byte_mapping() {
    let s: &str = "abcd";
    let inc: String = s.bytes().
        collect::<Vec<u8>>().
        iter().
        map(|b| {
            (b + 1) as char
        }).collect();
    assert!(&inc == "bcde");
}

#[test]
fn test_cycle() {
    let mut called = false;
    let mut i = 0;
    every_three_chars("abcdef", 3, |three_chars: &[u8]| { 
        if i == 0 { 
            let expected: &[u8; 3] = b"abc";
            assert_eq!(three_chars, expected);
        } else if i == 1 { 
            let expected: &[u8; 3] = b"def";
            assert_eq!(three_chars, expected);
        } else { 
            assert!(false);
        }
        i = i + 1;
        called = true;
    });
    assert!(called);
}


#[test]
fn test_base64_three_chars_simple_aaa() {
    let mut result: Vec<u8> = vec![0, 0, 0, 0];
	let input: Vec<u8> = "AAA".bytes().collect::<Vec<u8>>();
    base64_three_chars(&input, &mut result);
    assert_eq!(&result, b"QUFB");
}

#[test]
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


#[test]
fn test_base64_three_chars_symbolic() {

}

//#[test]
//fn test_base64_three_chars_simple2() {
//    let mut result: [u8; 4] = [0, 0, 0, 0];
//    base64_three_chars(b"A", &mut result);
//    assert_eq!(&result, b"QUFB");
//}


