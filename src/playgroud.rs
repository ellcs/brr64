use std::io::Read;


fn gen_vec(i: u8) -> Vec<u8> {
    (0..i).collect()
}

#[test]
fn test_gen_vec() {
    assert_eq!(0, gen_vec(0).len());
    assert_eq!(1, gen_vec(1).len());
    assert_eq!(2, gen_vec(2).len());
    assert_eq!(99, gen_vec(99).len());
}

#[test]
fn play_gen_vec_match() {
    match (gen_vec(0)) {
        (vec) if vec.len() > 0 => {
            assert!(false);
        },
        _ => {
            assert!(true);
        }
    }

    match (gen_vec(1)) {
        (vec) if vec.len() > 0 => {
            assert!(true);
        },
        _ => {
            assert!(true);
        }
    }

    (0..10).into_iter().map(|i|{
        i as u8
    }).for_each(|i|{
        let vec = gen_vec(1);
        vec.iter().for_each(|m| {
        });
    });
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
enum Tripplet {
    Symbolic,
    Real(Vec<u8>)
}

#[derive(Debug)]
enum SymbolicU8 {
    Symbolic,
    Real(u8)
}

enum Char64 {
    Sym,
    Real(u8)
}

#[test]
fn test_eq_vecs() {
    //let expe : Vec<u8> = vec![];
    // assert_eq!(expe, vec![1]);
    //assert_eq!(Tripplet::Real(vec![1,2,3]), Tripplet::Real(vec![1,2]));
}


#[test]
fn test_retain_in_vec() {
    let mut vec : Vec<u8> = vec![1,2,3,4];
    vec.retain(|byte| {
        byte.rem_euclid(2) == 0
    });
    assert_eq!(vec![2,4], vec);
}


#[test]
fn test_readfile() {
   let mut file = std::fs::File::open("/etc/passwd").unwrap(); 
   let mut buffer = String::new();
   file.read_to_string(&mut buffer).unwrap();
   assert!(buffer.contains("ellcs"));
}

struct Person {
    name: String,
    age: u32
}


#[test]
fn test_match() {
    let person = Person {
        name: String::from("Bob"),
        age: 18
    };
    match person {
        Person { name, age } if age >= 18 => {
            assert!(true);
        },
        // default
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_symbolic_u8_matched() {
    let matchme : Vec<Char64> = vec![Char64::Sym, Char64::Real(5), Char64::Real(5)];
    matchme.chunks(3_usize).for_each(|s| {
        match s {
            [Char64::Real(_r1), Char64::Real(_r2), Char64::Real(_r3)] => {},
            [Char64::Sym,       Char64::Real(_r2), Char64::Real(_r3)] => {},
            [Char64::Sym,       Char64::Sym,       Char64::Real(_r3)] => {},
            [Char64::Real(_r1), Char64::Sym,       Char64::Sym]      => {},
            [Char64::Real(_r1), Char64::Real(_r2), Char64::Sym]      => {},
            _ => panic!()
        }
    });
}


#[test]
fn test_symbolic_u8() {
    let mut vec : Vec<SymbolicU8> = vec![];
    vec.push(SymbolicU8::Symbolic);
    vec.push(SymbolicU8::Real(99));
    println!("{:?}", vec);
}


#[test]
fn test_tripplet() {
    let mut tripplets : Vec<Tripplet> = vec![];
    tripplets.push(Tripplet::Symbolic);
    tripplets.push(Tripplet::Real(vec![0, 0, 0, 0]));
    println!("{:?}", tripplets);
}

#[test]
fn test_bytes_and_bites() {
    let _empty: Vec<&[u8]> = vec!(b"", b"=a==", b"==a=");
    let _b = b"\xff".bytes();
    //b.asd();
    //assert_eq!(b"".bytes(), b"");
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
fn test_rev_str() {
    let s: &str = "asdf";
    let r: String = s.chars().rev().collect();
    assert!(&r == "fdsa");
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
fn test_extend() {
    let mut output = String::from("aa");
    let i = (3 - (output.len() % 3)) % 3;
    output.extend(std::iter::repeat('a').take(i));
    assert_eq!(output, "aaa");

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
fn test_modulo() {
    assert_eq!(1_i8, (-1_i8).rem_euclid(2_i8));
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

fn every_three_chars<F: FnMut(&[u8])>(s: &str, steps: u8, mut f: F) {
    s.bytes().
        collect::<Vec<u8>>().
        chunks(steps as usize).
        for_each(|c| {
            f(c)
        });
}
