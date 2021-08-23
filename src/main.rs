mod convert;
mod args;
mod symbolic_base_bro;

use std::env;


fn main() {
    //let mut input = String::new();
    let arg = env::args().nth(1).expect("Please provide string");

    let options = args::Options { 
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
