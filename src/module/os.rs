use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "os")]
extern {
    pub fn tmpdir() -> JsString;
}
