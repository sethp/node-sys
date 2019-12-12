use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "repl")]
extern {
    #[wasm_bindgen(js_name = "REPLServer", extends = Object)]
    #[derive(Clone, Debug, PartialEq)]
    pub type ReplServer;
}
