use structopt::StructOpt;
use std::path::PathBuf;


#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Options {

    #[structopt(flatten)]
    pub convert_options: ConvertOptions,

    /// Specifies which stream is used to search for the encoded search term.
    //#[structopt(short = "f", long, default_value = "-")]
    #[structopt(short = "f", long, parse(from_os_str), default_value = "-")]
    pub input_file: PathBuf,

    /// Prints a regular expression which can be used for grep. 
    #[structopt(short = "p", long)]
    pub print_regex: bool,

    // following field have to be set manually, because `structopt` can't set them.
    #[structopt(skip)]
    pub stdin_tty: bool,

    // following field have to be set manually, because `structopt` can't set them.
    #[structopt(skip)]
    pub stdout_tty: bool,

    /// Provided input text to brr64.
    #[structopt()]
    pub search_term: String,
}


#[derive(StructOpt, Debug)]
#[structopt()]
pub struct ConvertOptions {
    /// The base64 encoded string, you are looking for, might contain a newline. If you want to
    /// avoid the newline search, set this value to true. It's recommended to keep it on true,
    /// except for educational purposes.
    #[structopt(short = "n", long)]
    pub match_newlines: bool,

    /// Usually, when base64 encoding a string which is not three characters long, you will find
    /// padding at the end of the string, which is indicated as equal signs at the end. Caution:
    /// You will also have equal-characters at the beginning of the candidates. It's not
    /// recommended to have this option turned on when searching.
    #[structopt(short = "e", long)]
    pub print_equals: bool
}
