use crate::object::Timer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Timer)]
    pub type Timeout;
}
