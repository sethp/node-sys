mod helper {
    pub(crate) mod buffer {
        use node_sys::Buffer;

        pub fn new() -> Buffer {
            Buffer::from_array(Box::new([]))
        }
    }

    pub(crate) mod file {
        use js_sys::JsString;
        use node_sys::{fs, os, path};

        pub fn tmpdir() -> JsString {
            let prefix = &path::join({
                let mut val = vec![];
                val.push(os::tmpdir().into());
                val.push("node-sys".into());
                val.into_boxed_slice()
            });
            let options = Default::default();
            fs::mkdtemp_sync(prefix, options)
        }

        pub fn tmpfile(filename: &str) -> JsString {
            path::join({
                let mut val = vec![];
                val.push(tmpdir().into());
                val.push(filename.into());
                val.into_boxed_slice()
            })
        }
    }

    pub(crate) mod immediate {
        use node_sys::{timers, Immediate};
        use wasm_bindgen::{prelude::*, JsCast};

        pub fn new() -> Immediate {
            let clo = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
            let fun = clo.as_ref().unchecked_ref();
            let immediate = timers::set_immediate(fun, Box::new([]));
            clo.forget();
            immediate
        }
    }
}

mod class {
    mod buffer {
        mod r#static {
            use js_sys::{ArrayBuffer, Uint8Array};
            use node_sys::Buffer;
            use wasm_bindgen_test::*;

            #[wasm_bindgen_test]
            fn alloc() {
                let size = Default::default();
                let fill = Buffer::from_array(Box::new([]));
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
                let buf = Buffer::from_array(Box::new([]));
                Buffer::compare_(&buf, &buf);
            }

            #[wasm_bindgen_test]
            fn concat() {
                let list = Box::new([]);
                let total_length = Default::default();
                Buffer::concat(list, total_length);
            }

            #[wasm_bindgen_test]
            fn from_array() {
                let array = Box::new([]);
                Buffer::from_array(array);
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
                assert!(Buffer::is_buffer(&Buffer::from_array(Box::new([]))));
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
            use node_sys::Buffer;
            use wasm_bindgen_test::*;

            #[wasm_bindgen_test]
            fn compare() {
                let buffer = crate::helper::buffer::new();
                let target = &buffer;
                let target_start = Default::default();
                let target_end = Default::default();
                let source_start = Default::default();
                let source_end = Default::default();
                buffer.compare(target, target_start, target_end, source_start, source_end);
            }

            #[wasm_bindgen_test]
            fn copy() {
                let buffer = crate::helper::buffer::new();
                let target = crate::helper::buffer::new();
                let target_start = Default::default();
                let target_end = Default::default();
                let source_start = Default::default();
                let source_end = Default::default();
                buffer.compare(&target, target_start, target_end, source_start, source_end);
            }

            #[wasm_bindgen_test]
            fn entries() {
                let buffer = crate::helper::buffer::new();
                buffer.entries();
            }

            #[wasm_bindgen_test]
            fn equals() {
                let buffer = crate::helper::buffer::new();
                buffer.equals(&buffer);
            }

            #[wasm_bindgen_test]
            fn fill() {
                let buffer = crate::helper::buffer::new();
                let value = &0.into();
                let offset = Default::default();
                let end = Default::default();
                let encoding = Default::default();
                buffer.fill(value, offset, end, encoding);
            }

            #[wasm_bindgen_test]
            fn includes() {
                let buffer = crate::helper::buffer::new();
                let value = &0.into();
                let offset = Default::default();
                let encoding = Default::default();
                buffer.includes(value, offset, encoding);
            }

            #[wasm_bindgen_test]
            fn index_of() {
                let buffer = crate::helper::buffer::new();
                let value = &0.into();
                let offset = Default::default();
                let encoding = Default::default();
                buffer.index_of(value, offset, encoding);
            }

            #[wasm_bindgen_test]
            fn keys() {
                let buffer = crate::helper::buffer::new();
                buffer.keys();
            }

            #[wasm_bindgen_test]
            fn last_index_of() {
                let buffer = crate::helper::buffer::new();
                let value = &0.into();
                let offset = Default::default();
                let encoding = Default::default();
                buffer.last_index_of(value, offset, encoding);
            }

            #[wasm_bindgen_test]
            fn length() {
                let buffer = crate::helper::buffer::new();
                buffer.length();
            }

            #[wasm_bindgen_test]
            fn read_big_int64_be() {
                let buffer = Buffer::alloc(8.into(), None, None);
                let offset = None;
                buffer.read_big_int64_be(offset);
            }

            #[wasm_bindgen_test]
            fn read_big_int64_le() {
                let buffer = Buffer::alloc(8.into(), None, None);
                let offset = None;
                buffer.read_big_int64_le(offset);
            }

            #[wasm_bindgen_test]
            fn read_big_uint64_be() {
                let buffer = Buffer::alloc(8.into(), None, None);
                let offset = None;
                buffer.read_big_uint64_be(offset);
            }

            #[wasm_bindgen_test]
            fn read_big_uint64_le() {
                let buffer = Buffer::alloc(8.into(), None, None);
                let offset = None;
                buffer.read_big_uint64_le(offset);
            }

            #[wasm_bindgen_test]
            fn read_double_be() {
                let buffer = Buffer::alloc(8.into(), None, None);
                let offset = None;
                buffer.read_double_be(offset);
            }

            #[wasm_bindgen_test]
            fn read_double_le() {
                let buffer = Buffer::alloc(8.into(), None, None);
                let offset = None;
                buffer.read_double_le(offset);
            }

            #[wasm_bindgen_test]
            fn read_float_be() {
                let buffer = Buffer::alloc(4.into(), None, None);
                let offset = None;
                buffer.read_float_be(offset);
            }

            #[wasm_bindgen_test]
            fn read_float_le() {
                let buffer = Buffer::alloc(4.into(), None, None);
                let offset = None;
                buffer.read_float_le(offset);
            }

            #[wasm_bindgen_test]
            fn read_int_be() {
                let buffer = Buffer::alloc(6.into(), None, None);
                let offset = 0.into();
                let length = 6;
                buffer.read_int_be(offset, length);
            }

            #[wasm_bindgen_test]
            fn read_int_le() {
                let buffer = Buffer::alloc(6.into(), None, None);
                let offset = 0.into();
                let length = 6;
                buffer.read_int_le(offset, length);
            }

            #[wasm_bindgen_test]
            fn read_uint_be() {
                let buffer = Buffer::alloc(6.into(), None, None);
                let offset = 0.into();
                let length = 6;
                buffer.read_uint_be(offset, length);
            }

            #[wasm_bindgen_test]
            fn read_uint_le() {
                let buffer = Buffer::alloc(6.into(), None, None);
                let offset = 0.into();
                let length = 6;
                buffer.read_uint_le(offset, length);
            }

            #[wasm_bindgen_test]
            fn read_int8() {
                let buffer = Buffer::alloc(1.into(), None, None);
                let offset = None;
                buffer.read_int8(offset);
            }

            #[wasm_bindgen_test]
            fn read_int16_be() {
                let buffer = Buffer::alloc(2.into(), None, None);
                let offset = None;
                buffer.read_int16_be(offset);
            }

            #[wasm_bindgen_test]
            fn read_int16_le() {
                let buffer = Buffer::alloc(2.into(), None, None);
                let offset = None;
                buffer.read_int16_le(offset);
            }

            #[wasm_bindgen_test]
            fn read_uint8() {
                let buffer = Buffer::alloc(1.into(), None, None);
                let offset = None;
                buffer.read_uint8(offset);
            }

            #[wasm_bindgen_test]
            fn read_uint16_be() {
                let buffer = Buffer::alloc(2.into(), None, None);
                let offset = None;
                buffer.read_uint16_be(offset);
            }

            #[wasm_bindgen_test]
            fn read_uint16_le() {
                let buffer = Buffer::alloc(2.into(), None, None);
                let offset = None;
                buffer.read_uint16_le(offset);
            }

            #[wasm_bindgen_test]
            fn read_uint32_be() {
                let buffer = Buffer::alloc(4.into(), None, None);
                let offset = None;
                buffer.read_uint32_be(offset);
            }

            #[wasm_bindgen_test]
            fn read_uint32_le() {
                let buffer = Buffer::alloc(4.into(), None, None);
                let offset = None;
                buffer.read_uint32_le(offset);
            }

            #[wasm_bindgen_test]
            fn slice() {
                let buffer = Buffer::alloc(1.into(), None, None);
                let start = None;
                let end = None;
                buffer.slice(start, end);
            }

            #[wasm_bindgen_test]
            fn subarray() {
                let buffer = Buffer::alloc(1.into(), None, None);
                let start = None;
                let end = None;
                buffer.subarray(start, end);
            }

            #[wasm_bindgen_test]
            fn swap16() {
                let buffer = Buffer::alloc(2.into(), None, None);
                buffer.swap16();
            }

            #[wasm_bindgen_test]
            fn swap32() {
                let buffer = Buffer::alloc(4.into(), None, None);
                buffer.swap32();
            }

            #[wasm_bindgen_test]
            fn swap64() {
                let buffer = Buffer::alloc(8.into(), None, None);
                buffer.swap64();
            }

            #[wasm_bindgen_test]
            fn to_json() {
                let buffer = crate::helper::buffer::new();
                buffer.to_json();
            }

            #[wasm_bindgen_test]
            fn to_string() {
                let buffer = crate::helper::buffer::new();
                let encoding = None;
                let start = None;
                let end = None;
                buffer.to_string(encoding, start, end);
            }

            #[wasm_bindgen_test]
            fn values() {
                let buffer = crate::helper::buffer::new();
                buffer.values();
            }

            #[wasm_bindgen_test]
            fn write() {
                let buffer = crate::helper::buffer::new();
                let string = &"".into();
                let offset = Default::default();
                let length = Default::default();
                let encoding = Default::default();
                buffer.write(string, offset, length, encoding);
            }

            #[wasm_bindgen_test]
            fn write_big_int64_be() {
                let buffer = Buffer::alloc(8.into(), None, None);
                let value = 0i64;
                let offset = Default::default();
                buffer.write_big_int64_be(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_big_int64_le() {
                let buffer = Buffer::alloc(8.into(), None, None);
                let value = 0i64;
                let offset = Default::default();
                buffer.write_big_int64_le(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_int8() {
                let buffer = Buffer::alloc(1.into(), None, None);
                let value = 0i8;
                let offset = Default::default();
                buffer.write_int8(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_int16_be() {
                let buffer = Buffer::alloc(2.into(), None, None);
                let value = 0i16;
                let offset = Default::default();
                buffer.write_int16_be(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_int16_le() {
                let buffer = Buffer::alloc(2.into(), None, None);
                let value = 0i16;
                let offset = Default::default();
                buffer.write_int16_le(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_int32_be() {
                let buffer = Buffer::alloc(4.into(), None, None);
                let value = 0i32;
                let offset = Default::default();
                buffer.write_int32_be(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_int32_le() {
                let buffer = Buffer::alloc(4.into(), None, None);
                let value = 0i32;
                let offset = Default::default();
                buffer.write_int32_le(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_big_uint64_be() {
                let buffer = Buffer::alloc(8.into(), None, None);
                let value = 0u64;
                let offset = Default::default();
                buffer.write_big_uint64_be(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_big_uint64_le() {
                let buffer = Buffer::alloc(8.into(), None, None);
                let value = 0u64;
                let offset = Default::default();
                buffer.write_big_uint64_le(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_double_be() {
                let buffer = Buffer::alloc(8.into(), None, None);
                let value = 0f64;
                let offset = Default::default();
                buffer.write_double_be(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_double_le() {
                let buffer = Buffer::alloc(8.into(), None, None);
                let value = 0f64;
                let offset = Default::default();
                buffer.write_double_le(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_float_be() {
                let buffer = Buffer::alloc(4.into(), None, None);
                let value = 0f32;
                let offset = Default::default();
                buffer.write_float_be(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_float_le() {
                let buffer = Buffer::alloc(4.into(), None, None);
                let value = 0f32;
                let offset = Default::default();
                buffer.write_float_le(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_uint8() {
                let buffer = Buffer::alloc(1.into(), None, None);
                let value = 0u8;
                let offset = Default::default();
                buffer.write_uint8(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_uint16_be() {
                let buffer = Buffer::alloc(2.into(), None, None);
                let value = 0u16;
                let offset = Default::default();
                buffer.write_uint16_be(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_uint16_le() {
                let buffer = Buffer::alloc(2.into(), None, None);
                let value = 0u16;
                let offset = Default::default();
                buffer.write_uint16_le(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_uint32_be() {
                let buffer = Buffer::alloc(4.into(), None, None);
                let value = 0u32;
                let offset = Default::default();
                buffer.write_uint32_be(value, offset);
            }

            #[wasm_bindgen_test]
            fn write_uint32_le() {
                let buffer = Buffer::alloc(4.into(), None, None);
                let value = 0u32;
                let offset = Default::default();
                buffer.write_uint32_le(value, offset);
            }
        }
    }

    mod immediate {
        use wasm_bindgen_test::*;

        #[wasm_bindgen_test]
        pub fn has_ref() {
            let immediate = crate::helper::immediate::new();
            immediate.has_ref();
        }

        #[wasm_bindgen_test]
        pub fn ref_() {
            let immediate = crate::helper::immediate::new();
            immediate.ref_();
        }

        #[wasm_bindgen_test]
        pub fn unref() {
            let immediate = crate::helper::immediate::new();
            immediate.unref();
        }
    }
}

mod interface {}

mod module {
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
        use node_sys::buffer;
        use wasm_bindgen_test::*;

        #[allow(non_snake_case)]
        #[wasm_bindgen_test]
        fn INSPECT_MAX_BYTES() {
            let _ = buffer::INSPECT_MAX_BYTES;
        }

        #[wasm_bindgen_test]
        fn constants() {
            let constants = &buffer::constants;
            let _ = constants.MAX_LENGTH();
            let _ = constants.MAX_STRING_LENGTH();
        }

        #[wasm_bindgen_test]
        fn k_max_length() {
            let _ = buffer::k_max_length;
        }

        #[wasm_bindgen_test]
        fn transcode() {
            let buffer = crate::helper::buffer::new();
            let encoding = &"utf8".into();
            buffer::transcode(&buffer, encoding, encoding);
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

    mod fs {
        use node_sys::fs;
        use wasm_bindgen::{prelude::*, JsCast};
        use wasm_bindgen_test::*;

        #[wasm_bindgen_test]
        fn access() {
            let path = &".".into();
            let mode = Default::default();
            let clo = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
            let fun = clo.as_ref().unchecked_ref();
            fs::access(path, mode, fun);
            clo.forget()
        }

        #[wasm_bindgen_test]
        fn access_sync() {
            let path = &".".into();
            let mode = Default::default();
            fs::access_sync(path, mode);
        }

        #[wasm_bindgen_test]
        fn append_file() {
            let path = &crate::helper::file::tmpfile("append_file.test");
            let data = &crate::helper::buffer::new();
            let options = Default::default();
            let clo = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
            let fun = clo.as_ref().unchecked_ref();
            fs::append_file(path, data, options, fun);
            clo.forget();
        }
    }

    mod os {
        use node_sys::os;
        use wasm_bindgen_test::*;

        #[wasm_bindgen_test]
        pub fn arch() {
            os::arch();
        }

        #[wasm_bindgen_test]
        pub fn constants() {
            let _ = os::constants;
        }

        #[wasm_bindgen_test]
        pub fn constants_dlopen() {
            let dlopen = os::constants.dlopen();
            // let _ = dlopen.RTLD_DEEPBIND();
            let _ = dlopen.RTLD_GLOBAL();
            let _ = dlopen.RTLD_LAZY();
            let _ = dlopen.RTLD_LOCAL();
            let _ = dlopen.RTLD_NOW();
        }

        #[wasm_bindgen_test]
        pub fn constants_errno() {
            let _ = os::constants.errno();
        }

        #[wasm_bindgen_test]
        pub fn constants_libuv() {
            let _ = os::constants.libuv();
        }

        #[wasm_bindgen_test]
        pub fn constants_priority() {
            let _ = os::constants.priority();
        }

        #[wasm_bindgen_test]
        pub fn constants_signal() {
            let _ = os::constants.signal();
        }

        #[wasm_bindgen_test]
        pub fn cpus() {
            os::cpus();
        }

        #[wasm_bindgen_test]
        pub fn endianness() {
            os::endianness();
        }

        #[wasm_bindgen_test]
        pub fn freemem() {
            os::freemem();
        }

        #[wasm_bindgen_test]
        pub fn get_priority() {
            let pid = Default::default();
            os::get_priority(pid);
        }

        #[wasm_bindgen_test]
        pub fn homedir() {
            os::homedir();
        }

        #[wasm_bindgen_test]
        pub fn hostname() {
            os::hostname();
        }

        #[wasm_bindgen_test]
        pub fn loadavg() {
            os::loadavg();
        }

        #[wasm_bindgen_test]
        pub fn network_interfaces() {
            os::network_interfaces();
        }

        #[wasm_bindgen_test]
        pub fn platform() {
            os::platform();
        }

        #[wasm_bindgen_test]
        pub fn release() {
            os::release();
        }

        #[wasm_bindgen_test]
        pub fn set_priority() {
            let priority = 0;
            os::set_priority(priority);
        }

        #[wasm_bindgen_test]
        pub fn set_priority_for_pid() {
            let pid = Default::default();
            let priority = 0;
            os::set_priority_for_pid(pid, priority);
        }

        #[wasm_bindgen_test]
        pub fn tmpdir() {
            os::tmpdir();
        }

        #[wasm_bindgen_test]
        pub fn totalmem() {
            os::totalmem();
        }

        #[wasm_bindgen_test]
        pub fn type_() {
            os::type_();
        }

        #[wasm_bindgen_test]
        pub fn uptime() {
            os::uptime();
        }

        #[wasm_bindgen_test]
        pub fn user_info() {
            let options = None;
            os::user_info(options);
        }

        #[allow(non_snake_case)]
        #[wasm_bindgen_test]
        pub fn EOL() {
            let _ = os::EOL.clone();
        }
    }

    mod path {
        use node_sys::path;
        use wasm_bindgen_test::*;

        #[wasm_bindgen_test]
        fn join() {
            path::join({
                let mut val = vec![];
                val.push("foo".into());
                val.push("bar".into());
                val.push("baz".into());
                val.into_boxed_slice()
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
}
