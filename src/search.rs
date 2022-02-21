//use crate::args;
use crate::symbolic_base_bro;
use crate::symbolic_base_bro::*;

use std::collections::VecDeque;
//use std::io::BufRead;
use std::io::Read;
use std::iter::FromIterator;

use log::{debug, error};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Search<'search> {
    pub location: u64,
    //context: Vec<u8>,
    pub current_candidate: VecDeque<&'search symbolic_base_bro::OutChar64>
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct PushSearch<'search> {
    byte_count: u64,
    pub search_stack: Vec<Search<'search>>,
    candidates: &'search symbolic_base_bro::Candidates
}

pub fn by_candidates<'search>(candidates: &'search symbolic_base_bro::Candidates) -> PushSearch {
    PushSearch { byte_count: 0_u64,
                 candidates, 
                 search_stack: Vec::new() }
}

pub fn find_in_stream<R: Read>(mut rdr: R, candidates: &symbolic_base_bro::Candidates) {
    const BUFFER_SIZE: usize = 1 << 8;

    let symbolic_base_bro::Candidates(f, s, t) = candidates;
    let filtered_f = f.iter().filter(|c| **c != symbolic_base_bro::OutChar64::Equals).cloned().collect();
    let filtered_s = s.iter().filter(|c| **c != symbolic_base_bro::OutChar64::Equals).cloned().collect();
    let filtered_t = t.iter().filter(|c| **c != symbolic_base_bro::OutChar64::Equals).cloned().collect();
    let filterd_candidates = symbolic_base_bro::Candidates(filtered_f, filtered_s, filtered_t); 


    let mut search = by_candidates(&filterd_candidates);
    let mut buffer_vec = Vec::with_capacity(BUFFER_SIZE);
    let mut operation = |bytes: &[u8]| {
        debug!("Pushing {:?}", bytes);
        if push_all(&mut search, bytes) {
            let found = search.search_stack.iter().find(|s| {
                s.current_candidate.is_empty()
            });
            println!("{:?}", found.unwrap());
        }
    };
    loop {
        match rdr.by_ref().take(BUFFER_SIZE as u64).read_to_end(&mut buffer_vec) {
            Err(err) => {
                error!("Error while reading file: {}", err);
                std::process::exit(1);
            },
            Ok(chunk_size) => {
                if chunk_size == 0 {
                    break;
                }
                operation(&buffer_vec);
                if chunk_size < BUFFER_SIZE {
                    break;
                }
                buffer_vec.clear();
            }
        }

    }
}


/// returns true if push has found a position.
pub fn push_all(push_search: &mut PushSearch, input: &[u8]) -> bool {
    input.iter().for_each(|byte| {
        debug!("new byte read (byte, search_stack size): {:?} \t{:?}", byte, push_search.search_stack.len());
        // drop not matching searches
        if *byte != b'\n' {
            push_search.search_stack.retain(|prev_search| {
                if let Some(outchar64) = prev_search.current_candidate.front() {
                    byte == *outchar64
                } else {
                    true
                }
            });
        }


        // add new search
        let symbolic_base_bro::Candidates(c1, c2, c3) = push_search.candidates;

        debug!("push_search.byte_count {}", push_search.byte_count);
        if byte == c1.first().unwrap() {
            debug!("new candidate: {:?}", c1);
            push_search.search_stack.push(Search {
                location: push_search.byte_count,
                current_candidate: VecDeque::from_iter(c1.iter())
            });
        } 
        if byte == c2.first().unwrap() {
            debug!("new candidate: {:?}", c2);
            push_search.search_stack.push(Search {
                location: push_search.byte_count,
                current_candidate: VecDeque::from_iter(c2.iter())
            });
        } 
        if byte == c3.first().unwrap() {
            debug!("new candidate: {:?}", c3);
            push_search.search_stack.push(Search {
                location: push_search.byte_count,
                current_candidate: VecDeque::from_iter(c3.iter())
            });
        } 

        // move existing searches further
        push_search.search_stack.iter_mut().for_each(|prev_search| {
            // skip newlines in base64
            if *byte != b'\n' {
                // pop_front returns an optional. we don't unwrap it, because
                // a fail would mean that the Vec is empty and we dont care
                // about empty Vecs.
                prev_search.current_candidate.pop_front();
            }
        });

        push_search.byte_count += 1;
    });
    // when a candidate of a search is empty, the search
    // has been completed
    push_search.search_stack.iter().any(|existing_search| {
        existing_search.current_candidate.is_empty()
    })
}

impl PartialEq<u8> for symbolic_base_bro::OutChar64 {
    fn eq(&self, other: &u8) -> bool {
        match self {
            symbolic_base_bro::OutChar64::Single(byte) => {
                other == byte
            },
            symbolic_base_bro::OutChar64::Multiple(bytes) => {
                bytes.iter().any(|byte| {
                    other == byte
                })
            },
            symbolic_base_bro::OutChar64::Equals => {
                true
                //*other == b'='
            }
        }
    }
}

impl PartialEq<symbolic_base_bro::OutChar64> for u8 {
    fn eq(&self, other: &symbolic_base_bro::OutChar64) -> bool {
        match other {
            symbolic_base_bro::OutChar64::Single(byte) => {
                self == byte
            },
            symbolic_base_bro::OutChar64::Multiple(bytes) => {
                bytes.iter().any(|byte| {
                    self == byte
                })
            },
            symbolic_base_bro::OutChar64::Equals => {
                //*self == b'='
                true
            }
        }
    }
}

#[test]
fn test_push_search_simple_positive() {
    let candidates = symbolic_base_bro::generate_candidates("asdf");
    let mut search = by_candidates(&candidates);
    let input_bytes = b"   YXNkZg==   ";
    let out = push_all(&mut search, input_bytes);
    assert!(out);
}

#[test]
fn test_push_search_content_appears_twice() {
    let candidates = symbolic_base_bro::generate_candidates("asdf");
    let mut search = by_candidates(&candidates);
    let input_bytes = b"   YXNkZg==  YXNkZg== ";
    let out = push_all(&mut search, input_bytes);
    assert!(out);

    let found : Vec<&Search> = search.search_stack.iter().filter(|s| {
        s.current_candidate.is_empty()
    }).collect();
    assert_eq!(found.len(), 2);
}




#[test]
fn test_push_search_simple_positive_containing_newline_at_end() {
    let candidates = symbolic_base_bro::generate_candidates("asdf");
    let mut search = by_candidates(&candidates);
    let input_bytes = b"YXNkZg==\n";
    let out = push_all(&mut search, input_bytes);
    assert!(out);
}

#[test]
fn test_push_search_simple_positive_containing_newline_inbetween() {
    let candidates = symbolic_base_bro::generate_candidates("asdf");
    let mut search = by_candidates(&candidates);
    let input_bytes = b"YXNk\nZg==\n";
    let out = push_all(&mut search, input_bytes);
    assert!(out);
}

#[test]
fn test_push_search_simple_negative() {
    let candidates = symbolic_base_bro::generate_candidates("asdf");
    let mut search = by_candidates(&candidates);
    let input_bytes = b"YXXNkZg==";
    let out = push_all(&mut search, input_bytes);
    assert!(!out);
}


#[test]
fn test_search_long_base64() {
    let candidates = symbolic_base_bro::generate_candidates("qwer");
    let mut search = by_candidates(&candidates);
    let input_bytes = b"ICAgICAgcXdlciA=";
    let out = push_all(&mut search, input_bytes);
    assert!(out);
}

#[test]
fn test_search_push_all_multiple_times() {
    let candidates = symbolic_base_bro::generate_candidates("asdf");
    let mut search = by_candidates(&candidates);
    let multi_input_bytes : Vec<&[u8]> = vec![b"YXN", b"kZ", b"g=="];
    assert!(!push_all(&mut search, multi_input_bytes[0]));
    assert!(!push_all(&mut search, multi_input_bytes[1]));
    // after third w have a match
    assert!(push_all(&mut search, multi_input_bytes[2]));
}

