use js_sys::{Error, JsString};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[wasm_bindgen]
pub struct AssertionErrorOptions {
    message: Option<JsString>,
    actual: JsValue,
    expected: JsValue,
    operator: JsString,
}

#[wasm_bindgen]
impl AssertionErrorOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        message: Option<JsString>,
        actual: JsValue,
        expected: JsValue,
        operator: JsString,
    ) -> AssertionErrorOptions {
        AssertionErrorOptions {
            message,
            actual,
            expected,
            operator,
        }
    }

    /// If provided, the error message is set to this value.
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> Option<JsString> {
        self.message.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_message(&mut self, message: Option<JsString>) {
        self.message = message;
    }

    /// The actual property on the error instance.
    #[wasm_bindgen(getter)]
    pub fn actual(&self) -> JsValue {
        self.actual.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_actual(&mut self, actual: JsValue) {
        self.actual = actual
    }

    /// The expected property on the error instance.
    #[wasm_bindgen(getter)]
    pub fn expected(&self) -> JsValue {
        self.expected.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_expected(&mut self, expected: JsValue) {
        self.expected = expected
    }

    /// The operator property on the error instance.
    #[wasm_bindgen(getter)]
    pub fn operator(&self) -> JsString {
        self.operator.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_operator(&mut self, operator: JsString) {
        self.operator = operator
    }
}

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
