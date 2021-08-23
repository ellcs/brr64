mod convert;
mod args;
mod symbolic_base_bro;

use structopt::StructOpt;
use std::env;


fn main() {
    let options = args::Options::from_args();

    let regex = convert::string_by_candidates(&symbolic_base_bro::generate_candidates(&options.input), &options);
    println!("{}", regex);
}
