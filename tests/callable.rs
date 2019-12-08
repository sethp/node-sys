mod assert {
    use node_sys::assert;
    use wasm_bindgen::prelude::*;
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
            Reflect::set(&this, &"a".into(), &1u32.into()).unwrap_throw();
            this
        };
        let snd = {
            let this = Object::new();
            Reflect::set(&this, &"a".into(), &"1".into()).unwrap_throw();
            this
        };
        let message = Default::default();
        if let Err(_err) = assert::deep_strict_equal(&fst, &snd, message) {
            // #[should_panic]
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

mod events {
    use node_sys::{events, events::EventEmitter, process};
    use wasm_bindgen::{prelude::*, JsCast};
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn once() {
        let emitter = EventEmitter::new();
        let promise = events::once(&emitter, "event");
        let clo = Closure::wrap(Box::new(move || {
            emitter.emit("event", Box::new([42.into()]));
        }) as Box<dyn Fn()>);
        process.next_tick(clo.as_ref().unchecked_ref(), Box::new([0.into()]));
        clo.forget();
        JsFuture::from(promise).await.unwrap_throw();
    }
}

mod path {
    use js_sys::Array;
    use node_sys::path;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn join() {
        path::join(&{
            let val = Array::new();
            val.push(&"foo".into());
            val.push(&"bar".into());
            val.push(&"baz".into());
            val
        });
    }
}

mod process {
    use node_sys::process;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn abort() {
        // if let Err(_err) = process.abort() {
        //     // #[should_panic]
        // }
        // FIXME
    }

    #[wasm_bindgen_test]
    fn arch() {
        process.arch();
    }

    #[wasm_bindgen_test]
    fn argv() {
        process.argv();
    }

    #[wasm_bindgen_test]
    fn argv0() {
        process.argv0();
    }

    // FIXME
    #[wasm_bindgen_test]
    fn connected() {
        // process.connected();
    }

    #[wasm_bindgen_test]
    fn cpu_usage() {
        let previous_value = Default::default();
        process.cpu_usage(previous_value);
    }

    #[wasm_bindgen_test]
    fn cwd() {
        process.cwd();
    }

    #[wasm_bindgen_test]
    fn debug_port() {
        process.debug_port();
    }

    // FIXME
    #[wasm_bindgen_test]
    fn disconnect() {
        // process.disconnect();
    }

    #[wasm_bindgen_test]
    fn domain() {
        use wasm_bindgen::{prelude::*, JsCast};
        match process.domain() {
            None => {},
            Some(domain) => {
                let clo = Closure::wrap(Box::new(|x: JsValue| x) as Box<dyn Fn(JsValue) -> JsValue>);
                let fun = clo.as_ref().unchecked_ref();
                domain.add_with_emitter(&process);
                domain.bind(&fun);
                domain.intercept(&fun);
                domain.remove_with_emitter(&process);
                domain.run(&fun, Box::new([0.into()]));
                clo.forget();
            },
        }
    }

    #[wasm_bindgen_test]
    fn emit_warning() {
        let warning = &<&str as Default>::default().into();
        let name = Default::default();
        let ctor = Default::default();
        process.emit_warning(warning, name, ctor);
    }

    #[wasm_bindgen_test]
    fn env() {
        process.env();
    }

    #[wasm_bindgen_test]
    fn exec_argv() {
        process.exec_argv();
    }

    #[wasm_bindgen_test]
    fn exec_path() {
        process.exec_path();
    }

    #[wasm_bindgen_test]
    fn exit() {
        // process.exit();
    }

    #[wasm_bindgen_test]
    fn exit_code() {
        process.exit_code();
    }

    #[wasm_bindgen_test]
    fn get_egid() {
        process.get_egid();
    }

    #[wasm_bindgen_test]
    fn get_euid() {
        process.get_euid();
    }

    #[wasm_bindgen_test]
    fn get_gid() {
        process.get_gid();
    }

    #[wasm_bindgen_test]
    fn get_groups() {
        process.get_groups();
    }

    #[wasm_bindgen_test]
    fn get_uid() {
        process.get_gid();
    }

    #[wasm_bindgen_test]
    fn hrtime() {
        process.hr_time();
    }

    #[wasm_bindgen_test]
    fn kill() {
        let pid = 999_999;
        if let Err(_err) = process.kill(pid) {}
    }

    #[wasm_bindgen_test]
    fn kill_with_signal_name() {
        let pid = 999_999;
        let signal_name = &<&str as Default>::default().into();
        if let Err(_err) = process.kill_with_signal_name(pid, signal_name) {}
    }

    #[wasm_bindgen_test]
    fn kill_with_signal_id() {
        let pid = 999_999;
        let signal_id = Default::default();
        if let Err(_err) = process.kill_with_signal_id(pid, signal_id) {}
    }

    #[wasm_bindgen_test]
    fn main_module() {
        match process.main_module() {
            None => {},
            Some(main_module) => {
                let _ = main_module.exports();
                let _ = main_module.require();
                let _ = main_module.id();
                let _ = main_module.filename();
                let _ = main_module.loaded();
                let _ = main_module.parent();
                let _ = main_module.children();
                let _ = main_module.paths();
            },
        }
    }

    #[wasm_bindgen_test]
    fn memory_usage() {
        let memory_usage = process.memory_usage();
        let _ = memory_usage.external();
        let _ = memory_usage.heap_used();
        let _ = memory_usage.heap_total();
        let _ = memory_usage.rss();
    }

    #[wasm_bindgen_test]
    fn next_tick() {
        use js_sys::Number;
        use wasm_bindgen::{prelude::*, JsCast};
        let clo = Closure::wrap(Box::new(move |lhs: Number, rhs: Number| {
            let res = lhs.value_of() + rhs.value_of();
            res.into()
        }) as Box<dyn Fn(Number, Number) -> Number>);
        let add = clo.as_ref().unchecked_ref();
        let lhs = 1.into();
        let rhs = 1.into();
        process.next_tick(&add, Box::new([lhs, rhs]));
        clo.forget();
    }

    #[wasm_bindgen_test]
    fn platform() {
        process.platform();
    }

    #[wasm_bindgen_test]
    fn release() {
        let process_release = process.release();
        let _ = process_release.name();
        let _ = process_release.source_url();
        let _ = process_release.headers_url();
        let _ = process_release.lib_url();
        let _ = process_release.lts();
    }

    // FIXME: see binding comment; might be undefined
    #[wasm_bindgen_test]
    fn send() {
        // use wasm_bindgen::prelude::*;
        // let message = JsValue::UNDEFINED;
        // let send_handle = Default::default();
        // let options = Default::default();
        // let callback = Default::default();
        // process.send(&message, send_handle, options, callback);
    }

    #[wasm_bindgen_test]
    fn set_egid() {
        if let Err(_err) = process.set_egid(0) {}
    }

    #[wasm_bindgen_test]
    fn set_euid() {
        if let Err(_err) = process.set_euid(0) {}
    }

    #[wasm_bindgen_test]
    fn set_gid() {
        if let Err(_err) = process.set_gid(0) {}
    }

    #[wasm_bindgen_test]
    fn set_groups() {
        if let Err(_err) = process.set_groups(0) {}
    }

    #[wasm_bindgen_test]
    fn set_uid() {
        if let Err(_err) = process.set_uid(0) {}
    }

    #[wasm_bindgen_test]
    fn version() {
        process.version();
    }

    #[wasm_bindgen_test]
    fn versions() {
        let versions = process.versions();
        let _ = versions.ares();
        let _ = versions.brotli();
        let _ = versions.cldr();
        let _ = versions.icu();
        let _ = versions.llhttp();
        let _ = versions.modules();
        let _ = versions.napi();
        let _ = versions.nghttp2();
        let _ = versions.node();
        let _ = versions.openssl();
        let _ = versions.tz();
        let _ = versions.unicode();
        let _ = versions.uv();
        let _ = versions.v8();
        let _ = versions.zlib();
    }
}
