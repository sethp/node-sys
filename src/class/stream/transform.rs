use crate::class::stream::Duplex;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "stream")]
extern {
    #[wasm_bindgen(extends = Object, extends = Duplex)]
    #[derive(Clone, Debug)]
    pub type Transform;
}
