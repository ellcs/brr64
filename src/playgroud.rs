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
fn test_rev_str() {
    let s: &str = "asdf";
    let r: String = s.chars().rev().collect();
    assert!(&r == "fdsa");
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
#[should_panic]
fn test_overflows() {
    let a: u8 = 255;
    let b: u8 = 1;
    fooadd(&a, &b);
}

fn fooadd(a: &u8, b: &u8) -> u8 {
   a + b 
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

fn every_three_chars<F: FnMut(&[u8])>(s: &str, steps: u8, mut f: F) -> () {
    s.bytes().
        collect::<Vec<u8>>().
        chunks(steps as usize).
        for_each(|c| {
            f(&c)
        });
}
