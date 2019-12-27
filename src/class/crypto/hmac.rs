use crate::class::stream::Transform;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern {
    #[wasm_bindgen(extends = Transform)]
    #[derive(Clone, Debug)]
    pub type Hmac;
}
