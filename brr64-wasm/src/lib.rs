/*
 * This file is published under the CC0 licence.
 *
 *      https://creativecommons.org/publicdomain/zero/1.0/deed.en
 *
 * Most of the code in this file has been copied from:
 *
 *      https://www.hellorust.com/demos/sha1/index.html
 */
use brr64::symbolic_base_bro;
use brr64::convert;
use brr64::args;

use wasm_bindgen::prelude::*;

#[derive(Clone)]
#[wasm_bindgen(js_name = ConvertOptions)]
pub struct ConvertOptionsWrapper(args::ConvertOptions);

#[wasm_bindgen(js_class = ConvertOptions)]
impl ConvertOptionsWrapper {

	#[wasm_bindgen(constructor)]
	pub fn new() -> ConvertOptionsWrapper {
		ConvertOptionsWrapper(args::ConvertOptions { dont_match_newlines: false, print_equals: false })
	}

    #[wasm_bindgen(setter)]
    pub fn set_dont_match_newlines(&mut self, value: bool) {
        self.0.dont_match_newlines = value;
    }

    #[wasm_bindgen(setter)]
    pub fn set_print_equals(&mut self, value: bool) {
        self.0.print_equals = value;
    }
}


#[wasm_bindgen]
pub fn candidates(input: String, options: &ConvertOptionsWrapper) -> String {
    let candidates = symbolic_base_bro::generate_candidates(&input);
    convert::regex_string_by_candidates(&candidates, &options.clone().0)
}

#[wasm_bindgen]
extern "C" {
        fn alert(s: &str);
}


#[wasm_bindgen]
pub fn run_alert(item: &str) {
        alert(&format!("This is WASM and {}", item));
}
