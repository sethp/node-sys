use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Process;

    pub static process: Process;

    #[wasm_bindgen(method)]
    pub fn cwd(this: &Process) -> JsString;
}
