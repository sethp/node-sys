use crate::interface::Process;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub static process: Process;
}
