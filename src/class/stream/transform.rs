use crate::class::stream::Duplex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "stream")]
extern {
    #[wasm_bindgen(extends = Duplex)]
    #[derive(Clone, Debug)]
    pub type Transform;
}
