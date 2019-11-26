use js_sys::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Error)]
    pub type ErrnoException;
}
