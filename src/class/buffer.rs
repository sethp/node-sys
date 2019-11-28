use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Uint8Array)]
    #[derive(Clone, Debug)]
    pub type Buffer;
}
