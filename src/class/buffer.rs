use js_sys::{Array, ArrayBuffer, JsString, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Uint8Array)]
    #[derive(Clone, Debug)]
    pub type Buffer;

    #[wasm_bindgen(static_method_of = Buffer)]
    pub fn alloc(size: usize, fill: Option<&Buffer>, encoding: Option<&JsString>) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "allocUnsafe")]
    pub fn alloc_unsafe(size: usize) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "byteLength")]
    pub fn byte_length(string: &JsString, encoding: Option<&JsString>) -> u32;

    // FIXME: compiler complains about
    #[wasm_bindgen(static_method_of = Buffer, js_name = "compare")]
    pub fn compare_(buf1: &Buffer, buf2: &Buffer) -> i32;

    #[wasm_bindgen(static_method_of = Buffer)]
    pub fn concat(list: &Array, total_length: Option<u32>) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "isBuffer")]
    pub fn is_buffer(value: &JsValue) -> bool;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "isEncoding")]
    pub fn is_encoding(encoding: &JsString) -> bool;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "from")]
    pub fn from_array(array: &Array) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "from")]
    pub fn from_array_buffer(buffer: &ArrayBuffer, byte_offset: Option<u32>, length: Option<u32>) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "from")]
    pub fn from_array_uint8(array: &Uint8Array) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "from")]
    pub fn from_string(string: &JsString, encoding: Option<&JsString>) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, getter, js_name = "poolSize")]
    pub fn pool_size() -> usize;

    #[wasm_bindgen(method)]
    pub fn compare(
        source: &Buffer,
        target: &Buffer,
        target_start: Option<usize>,
        target_end: Option<usize>,
        source_start: Option<usize>,
        source_end: Option<usize>,
    ) -> i32;
}
