use crate::event_emitter::EventEmitter;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type Process;

    pub static process: Process;

    #[wasm_bindgen(method)]
    pub fn cwd(this: &Process) -> JsString;
}
