use crate::interface::Timer;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Object, extends = Timer)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type Timeout;
}
