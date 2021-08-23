use crate::args;
use crate::symbolic_base_bro;
use crate::symbolic_base_bro::*;
//use regex::Regex;


pub fn string_by_candidates(candidates: &symbolic_base_bro::Candidates, options: &args::Options) -> String {
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

pub fn string_by_candidate(candidate: &Vec<OutChar64>, options: &args::Options) -> String {
    candidate.iter().map(|outchar| {
        let mut out = String::new();
        match outchar {
            OutChar64::Single(byte) => {
                out.push(*byte as char);
                if options.match_newlines {
                    out.push_str("\\n?");
                }
            },
            OutChar64::Multiple(bytes) => {
                out.push('(');
                bytes.iter().enumerate().for_each(|(i, b)| {
                    out.push(*b as char);
                    if bytes.len() - 1 != i {
                        out.push('|');
                    }
                });
                out.push(')');
                if options.match_newlines {
                    out.push_str("\\n?");
                }
            },
            OutChar64::Equals => { 
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
    let options = args::Options { 
        match_newlines: false, 
        print_equals: false,
        input: String::from("")
    };
    let result = string_by_candidates(&symbolic_base_bro::generate_candidates(&options.input), &options);
    // output
    let output = "(||)";
    assert_eq!(output, result);
}

#[test]
fn test_generate_candidates_regex_empty_with_equals() {
    // input
    let options = args::Options { 
        match_newlines: false, 
        print_equals: true ,
        input: String::from("")
    };
    let result = string_by_candidates(&symbolic_base_bro::generate_candidates(&options.input), &options);
    // output
    let output = "(|====|====)";
    assert_eq!(output, result);
}

#[test]
fn test_generate_candidates_regex_simple_a() {
    // input
    let options = args::Options { 
        match_newlines: false, 
        print_equals: false ,
        input: String::from("A")
    };
    let result = string_by_candidates(&symbolic_base_bro::generate_candidates(&options.input), &options);
    // output
    let first = "Q(Q|R|S|T|U|V|W|X|Y|Z|a|b|c|d|e|f)";
    let second = "(E|U|k|0)(E|F|G|H)";
    let third = "(B|F|J|N|R|V|Z|d|h|l|p|t|x|1|5|9)B";
    let output = format!("({}|{}|{})", first, second, third);
    assert_eq!(output, result);
}

#[test]
fn test_generate_candidates_regex_simple_a_with_newlines() {
    // input
    let options = args::Options { 
        match_newlines: true, 
        print_equals: false,
        input: String::from("A") 
    };
    let result = string_by_candidates(&symbolic_base_bro::generate_candidates(&options.input), &options);
    // output
    let first = "Q\\n?(Q|R|S|T|U|V|W|X|Y|Z|a|b|c|d|e|f)\\n?";
    let second = "(E|U|k|0)\\n?(E|F|G|H)\\n?";
    let third = "(B|F|J|N|R|V|Z|d|h|l|p|t|x|1|5|9)\\n?B\\n?";
    let output = format!("({}|{}|{})", first, second, third);
    assert_eq!(output, result);
}



#[test]
fn test_generate_candidates_regex_simple_a_with_equals() {
    // input
    let options = args::Options { 
        match_newlines: false, 
        print_equals: true, 
        input: String::from("A") 
    };
    let result = string_by_candidates(&symbolic_base_bro::generate_candidates(&options.input), &options);
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
