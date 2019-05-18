use js_sys::{Boolean, Error, Function, JsString, Object, RegExp};
use wasm_bindgen::prelude::*;

// AssertionError
#[wasm_bindgen(module = assert)]
extern "C" {
    #[wasm_bindgen(extends = Object, extends = Error)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type AssertionError;

    #[wasm_bindgen(constructor)]
    pub fn new(options: &Object) -> AssertionError;

    #[wasm_bindgen(method, getter = actual)]
    pub fn actual(this: &AssertionError) -> JsValue;

    #[wasm_bindgen(method, getter = code)]
    pub fn code(this: &AssertionError) -> JsString;

    #[wasm_bindgen(method, getter = expected)]
    pub fn expected(this: &AssertionError) -> JsValue;

    #[wasm_bindgen(method, getter = generatedMessage)]
    pub fn generated_message(this: &AssertionError) -> Boolean;

    #[wasm_bindgen(method, getter = operator)]
    pub fn operator(this: &AssertionError) -> JsString;
}

#[wasm_bindgen(module = assert)]
extern "C" {
    #[wasm_bindgen(catch, js_name = "deepStrictEqual")]
    pub fn deep_strict_equal(actual: &JsValue, expected: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "deepStrictEqual")]
    pub fn deep_strict_equal_with_error(
        actual: &JsValue,
        expected: &JsValue,
        message: &Error,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "deepStrictEqual")]
    pub fn deep_strict_equal_with_string(
        actual: &JsValue,
        expected: &JsValue,
        message: &JsString,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "strictEqual")]
    pub fn strict_equal(actual: &JsValue, expected: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "strictEqual")]
    pub fn strict_equal_with_error(
        actual: &JsValue,
        expected: &JsValue,
        message: &Error,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "strictEqual")]
    pub fn strict_equal_with_message(
        actual: &JsValue,
        expected: &JsValue,
        message: &JsString,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "throws")]
    pub fn throws_with_regexp(fun: &Function, error: &RegExp, message: &JsString);

    #[wasm_bindgen(catch, js_name = "throws")]
    pub fn throws_with_function(fun: &Function, error: &Function, message: &JsString);

    #[wasm_bindgen(catch, js_name = "throws")]
    pub fn throws_with_object(fun: &Function, error: &Object, message: &JsString);

    #[wasm_bindgen(catch, js_name = "throws")]
    pub fn throws_with_error(fun: &Function, error: &Error, message: &JsString);
}
