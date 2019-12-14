use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub static __dirname: JsString;

    pub static __filename: JsString;

    pub fn require(id: &JsString) -> JsValue;
}
