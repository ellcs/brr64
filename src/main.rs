use std::io;

const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(n) => {
      println!("{} bytes read", n);
      println!("{}", input);
    }
    Err(error) => println!("error: {}", error),
  }
}



fn every_three_chars<F: FnMut(&[u8])>(s: &str, steps: u8, mut f: F) -> () {
    s.bytes().
        collect::<Vec<u8>>().
        chunks(steps as usize).
        for_each(|c| {
            f(&c)
        });
}

fn base64_three_chars(chars: &[u8; 3], result: &mut [u8; 4]) {
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
    let mut result: [u8; 4] = [0, 0, 0, 0];
    base64_three_chars(b"AAA", &mut result);
    assert_eq!(&result, b"QUFB");
}

#[test]
fn test_base64_three_chars_simple_abc() {
    let mut result: [u8; 4] = [0, 0, 0, 0];
    base64_three_chars(b"abc", &mut result);
    assert_eq!(&result, b"YWJj");
}

//#[test]
//fn test_base64_three_chars_simple2() {
//    let mut result: [u8; 4] = [0, 0, 0, 0];
//    base64_three_chars(b"A", &mut result);
//    assert_eq!(&result, b"QUFB");
//}


