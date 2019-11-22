mod assert {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn assertion_error_options_new() {
        fn test() -> Result<(), JsValue> {
            let message = Default::default();
            let actual = 0u32.into();
            let expected = 1u32.into();
            let operator = "strictEqual".into();
            let options = node_sys::assert::AssertionErrorOptions::new(message, actual, expected, operator)?;
            assert_eq!(options.actual(), 0);
            assert_eq!(options.expected(), 1);
            assert_eq!(options.operator(), "strictEqual");
            Ok(())
        }
        test().unwrap()
    }
}
