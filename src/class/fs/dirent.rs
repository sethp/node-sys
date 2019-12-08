use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "fs")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type Dirent;
}
