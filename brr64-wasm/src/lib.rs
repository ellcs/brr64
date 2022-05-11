use brr64::symbolic_base_bro;
use brr64::convert;
use brr64::args;

use wasm_bindgen::prelude::*;
//use js_sys::{Function, Object, Reflect, WebAssembly};

#[derive(Clone)]
#[wasm_bindgen(js_name = ConvertOptions)]
pub struct ConvertOptionsWrapper { 
    wrapped: args::ConvertOptions
}

#[wasm_bindgen(js_class = ConvertOptions)]
impl ConvertOptionsWrapper {

	#[wasm_bindgen(constructor)]
	pub fn new() -> ConvertOptionsWrapper {
		ConvertOptionsWrapper {
            wrapped: args::ConvertOptions { dont_match_newlines: false, print_equals: false }
        }
	}

    #[wasm_bindgen(setter)]
    pub fn set_dont_match_newlines(&mut self, value: bool) {
        self.wrapped.dont_match_newlines = value;
    }

    #[wasm_bindgen(setter)]
    pub fn set_print_equals(&mut self, value: bool) {
        self.wrapped.print_equals = value;
    }

}


#[wasm_bindgen]
pub fn candidates(input: String, options: &ConvertOptionsWrapper) -> String {
    let candidates = symbolic_base_bro::generate_candidates(&input);
    convert::regex_string_by_candidates(&candidates, &options.wrapped)
}

