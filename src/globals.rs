use crate::class::console::Console;
use js_sys::{Function, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub static __dirname: JsString;

    pub static __filename: JsString;

    pub static console: Console;

    pub static exports: Object;

    pub static global: Object;

    pub static module: Object;

    // pub static process: Object; // FIXME: name collision

    #[wasm_bindgen(js_name = "queueMicrotask")]
    pub fn queue_microtask(callback: &Function);

    pub fn require(id: &JsString) -> JsValue;
}
