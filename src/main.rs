//mod playgroud;
//mod base64_encode;
mod convert;
mod symbolic_base_bro;

use std::env;


fn main() {
    //let mut input = String::new();
    let arg = env::args().nth(1).expect("Please provide string");

    let options = convert::Options { 
        match_newlines: true, 
        print_equals: false 
    };
    let regex = convert::string_by_candidates(&symbolic_base_bro::generate_candidates(&arg), &options);
    println!("{}", regex);

    //_args.get(0);
    //while let Ok(_n) = io::stdin().read_line(&mut input) {
    //    // rm -f \n
    //    input.pop();
    //    let output = base64_encode::base64_encode(&input);
    //    println!("{}", output);
    //    input.clear();
    //}
}


//fn base64_encode_symbolic(input: Vec<u8>) -> Vec<Vec<u8>> {
//    let mut input0: Vec<u8> = input.clone();
//    let mut input1: Vec<u8> = input.clone();
//    let mut input2: Vec<u8> = input.clone();
//
//    input1.insert(0, b'\0');
//    input2.insert(0, b'\0'); input2.insert(0, b'\0');
//
//    let mut out: Vec<u8> = vec![0, 0, 0, 0];
//
//    let mut outputs = vec![&mut input0, &mut input1, &mut input2].into_iter().map(|input| {
//        let i = (3 - (input.len() % 3)) % 3;
//        input.extend(std::iter::repeat(b'\0').take(i));
//
//        let mut cont: Vec<u8> = input.
//            chunks(3_usize).
//            fold(Vec::new(), |mut accu, three_chars: &[u8]| {
//                base64_encode::base64_three_chars(three_chars, &mut out);
//                accu.append(&mut out.clone());
//                accu
//            });
//
//        let len = cont.len();
//        cont.splice(len-i..len, std::iter::repeat(b'=').take(i).collect::<Vec<u8>>());
//        cont
//    }).collect::<Vec<Vec<u8>>>();
//    outputs[1].splice(0..1, std::iter::repeat(b'=').take(1).collect::<Vec<u8>>());
//    outputs[2].splice(0..2, std::iter::repeat(b'=').take(2).collect::<Vec<u8>>());
//    outputs
//}
//
//fn symbolic_test(input: &[u8], vecs: Vec<&[u8]>) {
//    let empty: Vec<Vec<u8>> = vecs.iter().map(|b| b.to_vec()).collect();
//    assert_eq!(base64_encode_symbolic(input.to_vec()), empty);
//}
//
//#[test]
//fn test_base64_symbolic_empty() {
//    symbolic_test(b"", vec!(b"", b"=A==", b"==A="));
//}
//
//#[test]
//fn test_base64_symbolic_a() {
//    let empty = vec!(b"YQ==".to_vec(), b"=GE=".to_vec(), b"==Bh".to_vec());
//    assert_eq!(base64_encode_symbolic(b"a".to_vec()), empty);
//}
//
//#[test]
//fn test_base64_symbolic_za() {
//    let empty = vec!(b"emE=".to_vec(), b"=Hph".to_vec(), b"==B6YQ==".to_vec());
//    assert_eq!(base64_encode_symbolic(b"za".to_vec()), empty);
//}
//
//#[test]
//fn test_base64_symbolic_az() {
//    let empty = vec!(b"YXo=".to_vec(), b"=GF6".to_vec(), b"==Bheg==".to_vec());
//    assert_eq!(base64_encode_symbolic(b"az".to_vec()), empty);
//}
//
//#[test]
//fn test_base64_symbolic_zzz() {
//    let empty = vec!(b"enp6".to_vec(), b"=Hp6eg==".to_vec(), b"==B6eno=".to_vec());
//    assert_eq!(base64_encode_symbolic(b"zzz".to_vec()), empty);
//}
//
//#[test]
//fn test_base64_symbolic_zzzz() {
//    let empty = vec!(b"enp6eg==".to_vec(), b"=Hp6eno=".to_vec(), b"==B6enp6".to_vec());
//    assert_eq!(base64_encode_symbolic(b"zzzz".to_vec()), empty);
//}
//
//#[test]
//fn test_base64_symbolic_change_me_zzzz() {
//    let empty = vec!(b"enp6eg==".to_vec(), b"=Hp6eno=".to_vec(), b"==B6enp6".to_vec());
//    assert_eq!(base64_encode_symbolic(b"zzzz".to_vec()), empty);
//}
//
