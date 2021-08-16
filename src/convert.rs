use crate::symbolic_base_bro;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Options {

    /// The base64 encoded string, you are looking for, might contain a newline. If you want to
    /// avoid the newline search, set this value to false. It's recommended to keep it on false.
    #[structopt(short, long)]
    pub match_newlines: bool,

    /// Usually, when base64 encoding a string which is not three characters long, you will find
    /// padding at the end of the string, which is indicated as equal signs at the end.
    #[structopt(short, long)]
    pub print_equals: bool
}

pub fn string_by_candidates(candidates: &symbolic_base_bro::Candidates, options: &Options) -> String {
        let mut out = String::new();
        out.push('(');
        let symbolic_base_bro::Candidates(first, second, third) = candidates;
        let str_candidates = vec![first, second, third].into_iter().map(|candidate| {
            string_by_candidate(candidate, options)
        }).collect::<Vec<String>>().join("|");
        out.push_str(&str_candidates);
        out.push(')');
        out
}

pub fn string_by_candidate(candidate: &Vec<symbolic_base_bro::OutChar64>, options: &Options) -> String {
    candidate.iter().map(|outchar| {
        let mut out = String::new();
        match outchar {
            symbolic_base_bro::OutChar64::Single(byte) => {
                out.push(*byte as char);
                if !options.match_newlines {
                    out.push_str("\n?");
                }
            },
            symbolic_base_bro::OutChar64::Multiple(bytes) => {
                out.push('(');
                bytes.iter().enumerate().for_each(|(i, b)| {
                    out.push(*b as char);
                    if !options.match_newlines {
                        out.push_str("\n?");
                    }
                    if bytes.len() - 1 != i {
                        out.push('|');
                    }
                });
                out.push(')');
            },
            symbolic_base_bro::OutChar64::Equals => { 
                if options.print_equals {
                    out.push('=');
                }
            }
        }
        out
    }).collect::<Vec<String>>().join("")
}

#[test]
fn test_generate_candidates_regex_empty() {
    // input
    let input = &String::from("");
    let options = Options { match_newlines: true, print_equals: false };
    let result = string_by_candidates(&symbolic_base_bro::generate_candidates(&input), &options);
    // output
    let output = "(||)";
    assert_eq!(output, result);
}

#[test]
fn test_generate_candidates_regex_empty_with_equals() {
    // input
    let input = &String::from("");
    let options = Options { match_newlines: true, print_equals: true };
    let result = string_by_candidates(&symbolic_base_bro::generate_candidates(&input), &options);
    // output
    let output = "(|====|====)";
    assert_eq!(output, result);
}

#[test]
fn test_generate_candidates_regex_simple_a() {
    // input
    let input = &String::from("A");
    let options = Options { match_newlines: true, print_equals: false };
    let result = string_by_candidates(&symbolic_base_bro::generate_candidates(&input), &options);
    // output
    let first = "Q(Q|R|S|T|U|V|W|X|Y|Z|a|b|c|d|e|f)";
    let second = "(E|U|k|0)(E|F|G|H)";
    let third = "(B|F|J|N|R|V|Z|d|h|l|p|t|x|1|5|9)B";
    let output = format!("({}|{}|{})", first, second, third);
    assert_eq!(output, result);
}

#[test]
fn test_generate_candidates_regex_simple_a_with_equals() {
    // input
    let input = &String::from("A");
    let options = Options { match_newlines: true, print_equals: true };
    let result = string_by_candidates(&symbolic_base_bro::generate_candidates(&input), &options);
    // output
    let first = "Q(Q|R|S|T|U|V|W|X|Y|Z|a|b|c|d|e|f)==";
    let second = "=(E|U|k|0)(E|F|G|H)=";
    let third = "==(B|F|J|N|R|V|Z|d|h|l|p|t|x|1|5|9)B";
    let output = format!("({}|{}|{})", first, second, third);
    assert_eq!(output, result);
}

#[test]
fn test_generate_candidates_sanity_checks() {
    assert_ne!(symbolic_base_bro::generate_candidates(&String::from("A")), symbolic_base_bro::generate_candidates(&String::from("B")));
    assert_eq!(symbolic_base_bro::generate_candidates(&String::from("A")), symbolic_base_bro::generate_candidates(&String::from("A")));
}
