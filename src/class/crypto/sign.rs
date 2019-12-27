use crate::class::stream::Writable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern {
    #[wasm_bindgen(extends = Writable)]
    #[derive(Clone, Debug)]
    pub type Sign;
}
