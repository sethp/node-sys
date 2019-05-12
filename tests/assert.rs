#![cfg(feature = "assert")]

use js_sys::{Error, Object, Reflect};
use node_sys::assert::{self, AssertionError};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn assertion_error_new() {
    // manual error
    let manual = assert::AssertionError::new(&{
        let opts = Object::new();
        Reflect::set(&opts, &"actual".into(), &18u32.into()).unwrap();
        Reflect::set(&opts, &"expected".into(), &29u32.into()).unwrap();
        Reflect::set(&opts, &"operator".into(), &"strictEqual".into()).unwrap();
        opts.into()
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
