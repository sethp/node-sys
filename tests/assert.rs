#![cfg(all(
    feature = "AssertionError",
    feature = "AssertionErrorOptions",
    feature = "Error",
    feature = "assert"
))]

use node_sys::{assert, AssertionError, AssertionErrorOptions, Cast, Error};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn assertion_error_new() {
    // manual error
    let manual = AssertionError::new_with_options(&{
        let mut options = AssertionErrorOptions::new();
        options.actual(&1u32.into());
        options.expected(&2u32.into());
        options.operator("strictEqual".into());
        options
    })
    .unwrap();
    // thrown error
    if let Err(thrown) = assert::strict_equal_with_opt_str(&1u32.into(), &2u32.into(), None) {
        assert!(thrown.is_instance_of::<AssertionError>());
        let thrown = thrown.unchecked_into::<AssertionError>();

        assert_eq!(thrown.cast::<Error>().name(), manual.message());
        assert_eq!(thrown.actual(), 1);
        assert_eq!(thrown.expected(), 2);
        assert_eq!(thrown.code(), "ERR_ASSERTION");
        assert_eq!(thrown.operator(), "strictEqual");
        assert_eq!(thrown.generated_message(), true);

        assert_eq!(
            thrown.cast::<Error>().message(),
            manual.cast::<Error>().message(),
        );
        assert_eq!(thrown.cast::<Error>().name(), manual.cast::<Error>().name());
        assert_eq!(thrown.actual(), manual.actual());
        assert_eq!(thrown.expected(), manual.expected());
        assert_eq!(thrown.operator(), manual.operator());
    }
}
