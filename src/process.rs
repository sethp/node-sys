use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = process)]
extern "C" {
    pub fn cwd() -> JsString;
}
