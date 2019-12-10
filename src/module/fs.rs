pub use crate::class::fs::*;
use js_sys::{Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "fs")]
extern {
    pub fn access(path: &JsString, mode: Option<i32>, callback: &Function);
}
