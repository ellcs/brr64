//use crate::args;
use crate::symbolic_base_bro;
use crate::symbolic_base_bro::*;

use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Search<'search> {
    pub location: u32,
    //search_stack: Vec<u8>,
    pub current_candidate: VecDeque<&'search OutChar64>
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct PushSearch<'search> {
    byte_count: u32,
    pub search_stack: Vec<Search<'search>>,
    candidates: &'search symbolic_base_bro::Candidates
}

pub fn by_candidates<'search>(candidates: &'search symbolic_base_bro::Candidates) -> PushSearch {
    PushSearch { byte_count: 0_u32,
                 candidates: candidates, 
                 search_stack: Vec::new() }
}

#[inline(always)]
fn matches_outchar64(match_byte: &u8, outchar64: &OutChar64) -> bool {
    match outchar64 {
        OutChar64::Single(byte) => {
            match_byte == byte
        },
        OutChar64::Multiple(bytes) => {
            bytes.iter().any(|byte| {
                match_byte == byte
            })
        },
        OutChar64::Equals => {
            *match_byte == ('=' as u8)
        }
    }
}

/// returns true if push has found a position.
pub fn push_all(push_search: &mut PushSearch, input: &[u8]) -> bool {
    input.iter().for_each(|byte| {
        // drop not matching searches
        push_search.search_stack.retain(|prev_search| {
            let outchar64 = prev_search.current_candidate.front().unwrap();
            matches_outchar64(&byte, &outchar64)
        });

        // add new search
        let symbolic_base_bro::Candidates(c1, c2, c3) = push_search.candidates;
        if matches_outchar64(byte, c1.first().unwrap()) {
            push_search.search_stack.push(Search {
                location: push_search.byte_count,
                current_candidate: VecDeque::from_iter(c1.iter())
            });
        } 
        if matches_outchar64(byte, c2.first().unwrap()) {
            push_search.search_stack.push(Search {
                location: push_search.byte_count,
                current_candidate: VecDeque::from_iter(c2.iter())
            });
        } 
        if matches_outchar64(byte, c3.first().unwrap()) {
            push_search.search_stack.push(Search {
                location: push_search.byte_count,
                current_candidate: VecDeque::from_iter(c3.iter())
            });
        }

        // move existing searches further
        push_search.search_stack.iter_mut().for_each(|prev_search| {
            prev_search.current_candidate.pop_front().unwrap();
        });

        println!("{:?} - {:?}", byte, push_search);
        push_search.byte_count += 1;
    });
    // when a candidate of a search is empty, the search
    // has been completed
    push_search.search_stack.iter().any(|existing_search| {
        existing_search.current_candidate.is_empty()
    })
}

#[test]
fn test_matches_outchar64() {
    // single
    assert!(matches_outchar64(&('Y' as u8), &OutChar64::Single('Y' as u8)));

    // multiple
    assert!(matches_outchar64(&('Y' as u8), &OutChar64::Multiple(vec!['Y' as u8])));

    // single
    assert!(matches_outchar64(&('=' as u8), &OutChar64::Equals));
}

#[test]
fn test_push_search_simple_positive() {
    let candidates = symbolic_base_bro::generate_candidates("asdf");
    let mut search = by_candidates(&candidates);
    let input_bytes = b"YXNkZg==";
    let out = push_all(&mut search, input_bytes);
    println!("{:?}", &search);
    assert!(out);
}

#[test]
fn test_push_search_simple_negative() {
    let candidates = symbolic_base_bro::generate_candidates("asdf");
    let mut search = by_candidates(&candidates);
    let input_bytes = b"YXXNkZg==";
    let out = push_all(&mut search, input_bytes);
    println!("{:?}", &search);
    assert!(!out);
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

    println!("");
}

