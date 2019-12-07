use crate::interface::Process;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    /// The process module provides information about, and control over, the current Node.js
    /// process.
    pub static process: Process;
}
