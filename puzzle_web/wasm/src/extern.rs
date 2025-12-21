#![allow(non_upper_case_globals)]

use wasm_bindgen::prelude::*;
use web_sys::Performance;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2)]
    pub static performance: Performance;

    #[wasm_bindgen(typescript_type = number)]
    pub type Number;
}
