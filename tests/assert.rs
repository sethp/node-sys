#![cfg(feature = "assert")]

use js_sys::Error;
use node_sys::{
    assert::{self, AssertionError},
    options,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn assertion_error_new() {
    use wasm_bindgen::prelude::*;
    // manual error
    let manual = assert::AssertionError::new(&options! {
        actual: 18u32,
        expected: 29u32,
        operator: "strictEqual",
    });
    // thrown error
    if let Err(thrown) = assert::strict_equal(&18u32.into(), &29u32.into(), None) {
        let thrown = thrown.unchecked_into::<AssertionError>();
        assert_eq!(manual.operator(), thrown.operator());
        assert_eq!(manual.actual(), thrown.actual());
        assert_eq!(manual.expected(), thrown.expected());
        assert_eq!(
            manual.unchecked_into::<Error>().message(),
            thrown.unchecked_into::<Error>().message()
        );
    }
}
