use js_sys::{Array, Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Function)]
    pub type RequireResolve;

    #[wasm_bindgen(method)]
    fn paths(this: &RequireResolve, request: &JsString) -> Option<Array>;
}
