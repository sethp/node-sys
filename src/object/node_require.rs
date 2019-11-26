use crate::object::NodeRequireFunction;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = NodeRequireFunction)]
    pub type NodeRequire;
}
