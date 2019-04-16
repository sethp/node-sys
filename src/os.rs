use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = os)]
extern "C" {
    pub fn arch() -> JsString;

    pub fn tmpdir() -> JsString;

    pub static EOL: JsString;
}
