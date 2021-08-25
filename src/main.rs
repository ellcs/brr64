//mod convert;
use brr64::args;
use brr64::symbolic_base_bro;
use brr64::convert;

use structopt::StructOpt;


fn main() {
    let options = args::Options::from_args();

    let candidates = symbolic_base_bro::generate_candidates(&options.input);
    let regex = convert::regex_string_by_candidates(&candidates, &options);
    println!("{}", regex);
}
