use crate::interface::Timer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Timer)]
    pub type Timeout;
}
