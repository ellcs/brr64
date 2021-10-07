use brr64::args;
use brr64::symbolic_base_bro;
use brr64::convert;
use brr64::playgroud;
use brr64::search;

use std::io::BufRead;
use std::io::Read;
use structopt::StructOpt;
use atty::Stream;

pub fn options() -> args::Options {
    let mut options = args::Options::from_args();
    options.stdin_tty = atty::is(Stream::Stdin);
    options.stdout_tty = atty::is(Stream::Stdout);
    options
}

pub fn find_in_stream<R: Read>(mut rdr: R, candidates: &symbolic_base_bro::Candidates) {
    const BUFFER_SIZE: usize = 1 << 8;

    let candidates = symbolic_base_bro::generate_candidates("asdf");
    let mut search = search::by_candidates(&candidates);
    let mut buffer_vec = Vec::with_capacity(BUFFER_SIZE);
    let mut operation = |bytes: &[u8]| {
        if search::push_all(&mut search, bytes) {
            let found = search.search_stack.iter().find(|s| {
                s.current_candidate.is_empty()
            });
            println!("{}", bytes.len());
        }
    };
    loop {
        match rdr.by_ref().take(BUFFER_SIZE as u64).read_to_end(&mut buffer_vec) {
            Err(err) => {
                eprintln!("Error while reading file: {}", err);
                std::process::exit(1);
            },
            Ok(chunk_size) => {
                if chunk_size == 0 {
                    break;
                }
                operation(&buffer_vec);
                if chunk_size < BUFFER_SIZE {
                    break;
                }
                buffer_vec.clear();
            }
        }

    }
}


fn main() {
    let options = options();
    let candidates = symbolic_base_bro::generate_candidates(&options.search_term);
    let regex = convert::regex_string_by_candidates(&candidates, &options.convert_options);
    
    if options.print_regex {
        println!("{}", regex);
    } else {
        let path = options.input_file.clone();
        if path.as_os_str() == "-" {
            find_in_stream(std::io::stdin(), &candidates);
        } else {
            match std::fs::File::open(&path) {
                Ok(file) => {
                    find_in_stream(file, &candidates);
                },
                Err(_err) => {
                    eprintln!("Could not open file: {}", path.display());
                    std::process::exit(1);
                }
            }
        }
    }
}
