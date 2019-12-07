pub use crate::interface::assertion_error_options::AssertionErrorOptions;
use js_sys::{Error, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "assert")]
extern {
    #[wasm_bindgen(extends = Error)]
    pub type AssertionError;

    #[wasm_bindgen(constructor)]
    pub fn new(options: AssertionErrorOptions) -> AssertionError;

    #[wasm_bindgen(catch, js_name = "deepStrictEqual")]
    pub fn deep_strict_equal(actual: &JsValue, expected: &JsValue, message: Option<&JsString>) -> Result<(), JsValue>;
}
