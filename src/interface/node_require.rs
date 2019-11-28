use crate::interface::NodeRequireFunction;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = NodeRequireFunction)]
    pub type NodeRequire;
}
