use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern {
    #[wasm_bindgen(extends = Object, js_name = "ECDH")]
    #[derive(Clone, Debug, PartialEq)]
    pub type Ecdh;
}
