mod assert {
    use node_sys::assert;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn assertion_error_options_new() {
        let message = Default::default();
        let actual = 0u32.into();
        let expected = 1u32.into();
        let operator = "strictEqual".into();
        let options = assert::AssertionErrorOptions::new(message, actual, expected, operator);
        assert_eq!(options.actual(), 0);
        assert_eq!(options.expected(), 1);
        assert_eq!(options.operator(), "strictEqual");
    }

    #[wasm_bindgen_test]
    fn assertion_error_new() {
        let message = Default::default();
        let actual = 0u32.into();
        let expected = 1u32.into();
        let operator = "strictEqual".into();
        let options = assert::AssertionErrorOptions::new(message, actual, expected, operator);
        let error = assert::AssertionError::new(options);
        let message = error.message();
        assert_eq!(message, "Expected values to be strictly equal:\n\n0 !== 1\n");
    }

    #[wasm_bindgen_test]
    fn deep_strict_equal() {
        use js_sys::{Object, Reflect};
        use node_sys::assert;
        let fst = {
            let this = Object::new();
            Reflect::set(&this, &"a".into(), &1u32.into()).unwrap();
            this
        };
        let snd = {
            let this = Object::new();
            Reflect::set(&this, &"a".into(), &"1".into()).unwrap();
            this
        };
        let message = Default::default();
        if let Err(_err) = assert::deep_strict_equal(&fst, &snd, message) {
            // #[should_panic]
            return;
        }
    }
}

mod async_hooks {
    use node_sys::async_hooks;
    use wasm_bindgen_test::*;

    mod helper {
        use node_sys::async_hooks;
        use wasm_bindgen::{prelude::*, JsCast};

        pub fn create_hook_callbacks() -> async_hooks::CreateHookCallbacks {
            let init = Closure::wrap(Box::new(move || {}) as Box<dyn Fn()>);
            let before = Closure::wrap(Box::new(move || {}) as Box<dyn Fn()>);
            let after = Closure::wrap(Box::new(move || {}) as Box<dyn Fn()>);
            let destroy = Closure::wrap(Box::new(move || {}) as Box<dyn Fn()>);
            let promise_resolve = Closure::wrap(Box::new(move || {}) as Box<dyn Fn()>);
            let callbacks = async_hooks::CreateHookCallbacks::new(
                init.as_ref().unchecked_ref(),
                before.as_ref().unchecked_ref(),
                after.as_ref().unchecked_ref(),
                destroy.as_ref().unchecked_ref(),
                promise_resolve.as_ref().unchecked_ref(),
            );
            init.forget();
            before.forget();
            after.forget();
            destroy.forget();
            promise_resolve.forget();
            callbacks
        }
    }

    #[wasm_bindgen_test]
    fn create_hook_callbacks_new() {
        helper::create_hook_callbacks();
    }

    #[wasm_bindgen_test]
    fn create_hook() {
        let callbacks = helper::create_hook_callbacks();
        async_hooks::create_hook(callbacks);
    }
}
