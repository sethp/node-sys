pub use crate::interface::assertion_error_options::AssertionErrorOptions;
use js_sys::{Error, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "assert")]
extern {
    /// Indicates the failure of an assertion. All errors thrown by the assert module will be
    /// instances of the AssertionError class.
    #[wasm_bindgen(extends = Error)]
    pub type AssertionError;

    #[wasm_bindgen(constructor)]
    pub fn new(options: AssertionErrorOptions) -> AssertionError;

    /// Tests for deep equality between the actual and expected parameters. "Deep" equality means
    /// that the enumerable "own" properties of child objects are recursively evaluated also by the
    /// following rules.
    #[wasm_bindgen(catch, js_name = "deepStrictEqual")]
    pub fn deep_strict_equal(actual: &JsValue, expected: &JsValue, message: Option<&JsString>) -> Result<(), JsValue>;
}
