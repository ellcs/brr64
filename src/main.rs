use brr64::args;
use brr64::symbolic_base_bro;
use brr64::convert;

use std::io::{Read};
use structopt::StructOpt;
use atty::Stream;

pub fn options() -> args::Options {
    let mut options = args::Options::from_args();
    options.stdin_tty = atty::is(Stream::Stdin);
    options.stdout_tty = atty::is(Stream::Stdout);
    options
}

pub fn open_io(options: &args::Options) -> Box<dyn std::io::Read + 'static> {
    let path = options.input_file.clone();
    if path.as_os_str() == "-" {
        Box::new(std::io::stdin())
    } else {
        match std::fs::File::open(&path) {
            Ok(file) => Box::new(file),
            Err(_err) => {
                eprintln!("Could not open file: {}", path.display());
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let options = options();
    let candidates = symbolic_base_bro::generate_candidates(&options.search_term);
    let regex = convert::regex_string_by_candidates(&candidates, &options.convert_options);

    
    
    let buf = open_io(&options);
    let mut buffer = [0; 5];

    // read at most five bytes
    let mut handle = buf.take(5);

    handle.read(&mut buffer).unwrap();

    println!("{:?}", buffer);

    if options.print_regex {
        println!("{}", regex);
    }
}
