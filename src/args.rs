use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Options {

    /// The base64 encoded string, you are looking for, might contain a newline. If you want to
    /// avoid the newline search, set this value to true. It's recommended to keep it on true,
    /// except for educational purposes.
    #[structopt(short, long)]
    pub match_newlines: bool,

    /// Usually, when base64 encoding a string which is not three characters long, you will find
    /// padding at the end of the string, which is indicated as equal signs at the end. Caution:
    /// You will also have equal-characters at the beginning of the candidates. It's not
    /// recommended to have this option turned on when searching.
    #[structopt(short, long)]
    pub print_equals: bool
}
