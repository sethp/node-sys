use js_sys::{Function, Object, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type EventEmitter;

    #[wasm_bindgen(method)]
    pub fn on(this: &EventEmitter, event: JsString, cb: &Function);
}
