#![allow(non_upper_case_globals)]

use wasm_bindgen::prelude::*;
use web_sys::Performance;

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    pub static performance: Performance;

    #[wasm_bindgen(typescript_type = number)]
    pub type Number;
}
