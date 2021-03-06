use brr64::args;
use brr64::symbolic_base_bro;
use brr64::convert;
use brr64::search;

#[macro_use]
extern crate log;
use structopt::StructOpt;
use atty::Stream;

pub fn options() -> args::Options {
    let mut options = args::Options::from_args();
    options.stdin_tty = atty::is(Stream::Stdin);
    options.stdout_tty = atty::is(Stream::Stdout);
    options
}


fn main() {
    env_logger::init();
    let options = options();
    let candidates = symbolic_base_bro::generate_candidates(&options.search_term);
    let regex = convert::regex_string_by_candidates(&candidates, &options.convert_options);
    info!("Generated candidate is {:?}", regex);
    
    if options.print_regex {
        println!("{}", regex);
    } else {
        let path = options.input_file;
        if let Some(search_path) = path {
            match std::fs::File::open(&search_path) {
                Ok(file) => {
                    debug!("Reading from {}", search_path.display());
                    search::find_in_stream(file, &candidates);
                },
                Err(_err) => {
                    error!("Could not open file: {}", search_path.display());
                    std::process::exit(1);
                }
            }
        } else {
            debug!("Reading from stdin...");
            search::find_in_stream(std::io::stdin(), &candidates);
        }
    }
}
