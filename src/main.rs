mod convert;
mod args;
mod symbolic_base_bro;

use structopt::StructOpt;


fn main() {
    let options = args::Options::from_args();

    let candidates = symbolic_base_bro::generate_candidates(&options.input);
    let regex = convert::regex_string_by_candidates(&candidates, &options);
    println!("{}", regex);
}
