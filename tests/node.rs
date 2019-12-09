mod assert {
    use js_sys::{Object, Promise, Reflect};
    use node_sys::assert;
    use wasm_bindgen::{prelude::*, JsCast};
    use wasm_bindgen_futures::JsFuture;
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

    #[wasm_bindgen_test]
    async fn does_not_reject_function() {
        let clo = Closure::wrap(Box::new(|| Promise::resolve(&JsValue::UNDEFINED)) as Box<dyn Fn() -> Promise>);
        let fun = clo.as_ref().unchecked_ref();
        let promise = assert::does_not_reject_function(&fun, None, None);
        clo.forget();
        JsFuture::from(promise).await.unwrap_throw();
    }

    #[wasm_bindgen_test]
    async fn does_not_reject_promise() {
        JsFuture::from(assert::does_not_reject_promise(
            &Promise::resolve(&JsValue::UNDEFINED),
            None,
            None,
        ))
        .await
        .unwrap_throw();
    }

    #[wasm_bindgen_test]
    fn does_not_throw() {
        let clo = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        let fun = clo.as_ref().unchecked_ref();
        assert::does_not_throw(&fun, None, None).unwrap_throw();
    }

    #[wasm_bindgen_test]
    fn fail() {
        if let Err(_err) = assert::fail(None) {}
    }

    #[wasm_bindgen_test]
    fn if_error_fail() {
        if let Err(_err) = assert::if_error(&0.into()) {}
    }

    #[wasm_bindgen_test]
    fn if_error_pass() {
        assert::if_error(&JsValue::NULL).unwrap_throw();
    }

    #[wasm_bindgen_test]
    fn not_deep_strict_equal() {
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
        assert::not_deep_strict_equal(&fst, &snd, message).unwrap_throw();
    }

    #[wasm_bindgen_test]
    fn not_strict_equal() {
        assert::not_strict_equal(&0.into(), &1.into(), None).unwrap_throw();
    }

    #[wasm_bindgen_test]
    fn ok_fail() {
        if let Err(_err) = assert::ok(&JsValue::NULL, None) {}
    }

    #[wasm_bindgen_test]
    fn ok_pass() {
        assert::ok(&1.into(), None).unwrap_throw();
    }

    #[wasm_bindgen_test]
    async fn rejects_function() {
        let clo = Closure::wrap(Box::new(|| Promise::reject(&JsValue::UNDEFINED)) as Box<dyn Fn() -> Promise>);
        let fun = clo.as_ref().unchecked_ref();
        let promise = assert::rejects_function(&fun, None, None);
        clo.forget();
        JsFuture::from(promise).await.unwrap_throw();
    }

    #[wasm_bindgen_test]
    async fn rejects_promise() {
        JsFuture::from(assert::rejects_promise(
            &Promise::reject(&JsValue::UNDEFINED),
            None,
            None,
        ))
        .await
        .unwrap_throw();
    }

    #[wasm_bindgen_test]
    fn strict_equal() {
        assert::strict_equal(&0.into(), &0.into(), None).unwrap_throw();
    }

    #[wasm_bindgen_test]
    fn throws() {
        use js_sys::JsString;
        let clo = Closure::wrap(Box::new(|| {
            let val = JsValue::UNDEFINED;
            let str = val.unchecked_into::<JsString>(); // coerce undefined into String
            str.length();
        }) as Box<dyn Fn()>);
        let fun = clo.as_ref().unchecked_ref();
        assert::throws(&fun, None, None).unwrap_throw();
    }
}

mod async_hooks {
    use node_sys::async_hooks;
    use wasm_bindgen_test::*;

    mod helper {
        use node_sys::async_hooks;
        use wasm_bindgen::{prelude::*, JsCast};

        pub fn create_hook_callbacks() -> async_hooks::CreateHookCallbacks {
            let init = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
            let before = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
            let after = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
            let destroy = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
            let promise_resolve = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
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

mod buffer {
    pub mod helper {
        use js_sys::Array;
        use node_sys::Buffer;

        pub fn buffer() -> Buffer {
            Buffer::from_array(&Array::new())
        }
    }

    mod r#static {
        use js_sys::{Array, ArrayBuffer, Uint8Array};
        use node_sys::Buffer;
        use wasm_bindgen_test::*;

        #[wasm_bindgen_test]
        fn alloc() {
            let size = Default::default();
            let fill = Buffer::from_array(&Array::new());
            let encoding = Default::default();
            Buffer::alloc(size, Some(&fill), encoding);
        }

        #[wasm_bindgen_test]
        fn alloc_unsafe() {
            let size = Default::default();
            Buffer::alloc_unsafe(size);
        }

        #[wasm_bindgen_test]
        fn byte_length() {
            let string = &"".into();
            let encoding = Default::default();
            Buffer::byte_length(&string, encoding);
        }

        #[wasm_bindgen_test]
        fn compare_() {
            let buf = Buffer::from_array(&Array::new());
            Buffer::compare_(&buf, &buf);
        }

        #[wasm_bindgen_test]
        fn concat() {
            let list = &Array::new();
            let total_length = Default::default();
            Buffer::concat(list, total_length);
        }

        #[wasm_bindgen_test]
        fn from_array() {
            let array = Array::new();
            Buffer::from_array(&array);
        }

        #[wasm_bindgen_test]
        fn from_array_buffer() {
            let buffer = ArrayBuffer::new(0);
            let byte_offset = Default::default();
            let length = Default::default();
            Buffer::from_array_buffer(&buffer, byte_offset, length);
        }

        #[wasm_bindgen_test]
        fn from_array_uint8() {
            let array = Uint8Array::new_with_length(0);
            Buffer::from_array_uint8(&array);
        }

        #[wasm_bindgen_test]
        fn from_string() {
            let string = &"".into();
            let encoding = Default::default();
            Buffer::from_string(string, encoding);
        }

        #[wasm_bindgen_test]
        fn is_buffer() {
            assert!(Buffer::is_buffer(&Buffer::from_array(&Array::new())));
        }

        #[wasm_bindgen_test]
        fn is_encoding() {
            assert!(Buffer::is_encoding(&"utf8".into()));
        }

        #[wasm_bindgen_test]
        fn pool_size() {
            Buffer::pool_size();
        }
    }

    mod instance {
        use wasm_bindgen_test::*;

        #[wasm_bindgen_test]
        fn compare() {
            let buffer = crate::buffer::helper::buffer();
            let target = &buffer;
            let target_start = Default::default();
            let target_end = Default::default();
            let source_start = Default::default();
            let source_end = Default::default();
            buffer.compare(target, target_start, target_end, source_start, source_end);
        }

        #[wasm_bindgen_test]
        fn copy() {
            let buffer = crate::buffer::helper::buffer();
            let target = crate::buffer::helper::buffer();
            let target_start = Default::default();
            let target_end = Default::default();
            let source_start = Default::default();
            let source_end = Default::default();
            buffer.compare(&target, target_start, target_end, source_start, source_end);
        }

        #[wasm_bindgen_test]
        fn entries() {
            let buffer = crate::buffer::helper::buffer();
            buffer.entries();
        }

        #[wasm_bindgen_test]
        fn equals() {
            let buffer = crate::buffer::helper::buffer();
            buffer.equals(&buffer);
        }

        #[wasm_bindgen_test]
        fn fill() {
            let buffer = crate::buffer::helper::buffer();
            let value = &0.into();
            let offset = Default::default();
            let end = Default::default();
            let encoding = Default::default();
            buffer.fill(value, offset, end, encoding);
        }

        #[wasm_bindgen_test]
        fn includes() {
            let buffer = crate::buffer::helper::buffer();
            let value = &0.into();
            let offset = Default::default();
            let encoding = Default::default();
            buffer.includes(value, offset, encoding);
        }
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
        let clo = Closure::wrap(Box::new(|lhs: Number, rhs: Number| {
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
    fn uptime() {
        process.uptime();
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
