use brr64::args;
use brr64::symbolic_base_bro;
use brr64::convert;
//use brr64::playgroud;
use brr64::search;

//use log::{debug, error, info, warn};
use log::error;
use structopt::StructOpt;
use atty::Stream;

pub fn options() -> args::Options {
    let mut options = args::Options::from_args();
    options.stdin_tty = atty::is(Stream::Stdin);
    options.stdout_tty = atty::is(Stream::Stdout);
    options
}


fn main() {
    let options = options();
    let candidates = symbolic_base_bro::generate_candidates(&options.search_term);
    
    if options.print_regex {
        let regex = convert::regex_string_by_candidates(&candidates, &options.convert_options);
        println!("{}", regex);
    } else {
        let path = options.input_file.clone();
        if path.as_os_str() == "-" {
            search::find_in_stream(std::io::stdin(), &candidates);
        } else {
            match std::fs::File::open(&path) {
                Ok(file) => {
                    search::find_in_stream(file, &candidates);
                },
                Err(_err) => {
                    error!("Could not open file: {}", path.display());
                    std::process::exit(1);
                }
            }
        }
    }
}
