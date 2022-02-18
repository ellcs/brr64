use structopt::StructOpt;
use std::path::PathBuf;


#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Options {

    #[structopt(flatten)]
    pub convert_options: ConvertOptions,

    /// Prints a regular expression which can be used for grep. 
    #[structopt(short = "p", long)]
    pub print_regex: bool,

    // following field has to be set manually, because `structopt` can't set them.
    #[structopt(skip)]
    pub stdin_tty: bool,

    // following field has to be set manually, because `structopt` can't set them.
    #[structopt(skip)]
    pub stdout_tty: bool,

    /// Provided input text to brr64.
    #[structopt()]
    pub search_term: String,

    /// Specifies which stream is used to search for the encoded search term.
    //#[structopt(short = "f", long, default_value = "-")]
    #[structopt(parse(from_os_str))]
    pub input_file: Option<PathBuf>,

}


#[derive(StructOpt, Debug)]
#[derive(Clone)]
#[structopt()]
pub struct ConvertOptions {
    /// A base64 encoded string can contain a newline at any position. `brr64` keeps these newlines
    /// in mind. It's recommended to avoid this flag. Educational purposes might be an exception.
    #[structopt(short = "n", long)]
    pub dont_match_newlines: bool,

    /// Usually, when base64 encoding a string which is not three characters long, you will find
    /// padding at the end of the string, which is indicated as equal signs at the end. Caution:
    /// You will also have equal-characters at the beginning of the candidates. It's not
    /// recommended to have this option turned on when searching.
    #[structopt(short = "e", long)]
    pub print_equals: bool
}
