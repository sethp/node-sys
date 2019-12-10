pub use crate::class::fs::*;
use js_sys::{Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "fs")]
extern {
    pub fn access(path: &JsString, mode: Option<u32>, callback: &Function);

    #[wasm_bindgen(js_name = "accessSync")]
    pub fn access_sync(path: &JsString, mode: Option<u32>);
}
