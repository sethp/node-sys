use js_sys::{Array, ArrayBuffer, JsString, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Uint8Array)]
    #[derive(Clone, Debug)]
    pub type Buffer;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = Buffer)]
    pub fn alloc(size: f64, fill: Option<&Buffer>, encoding: Option<&JsString>) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "allocUnsafe")]
    pub fn alloc_unsafe(size: f64) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "byteLength")]
    pub fn byte_length(string: &JsString, encoding: Option<&JsString>) -> f64;

    // FIXME: compiler complains about
    #[wasm_bindgen(static_method_of = Buffer, js_name = "compare")]
    pub fn compare_(buf1: &Buffer, buf2: &Buffer) -> i32;

    #[wasm_bindgen(static_method_of = Buffer)]
    pub fn concat(list: &Array, total_length: Option<f64>) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "isBuffer")]
    pub fn is_buffer(value: &JsValue) -> bool;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "isEncoding")]
    pub fn is_encoding(encoding: &JsString) -> bool;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "from")]
    pub fn from_array(array: &Array) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "from")]
    pub fn from_array_buffer(buffer: &ArrayBuffer, byte_offset: Option<f64>, length: Option<f64>) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "from")]
    pub fn from_array_uint8(array: &Uint8Array) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = "from")]
    pub fn from_string(string: &JsString, encoding: Option<&JsString>) -> Buffer;

    #[wasm_bindgen(static_method_of = Buffer, getter, js_name = "poolSize")]
    pub fn pool_size() -> f64;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn compare(
        this: &Buffer,
        target: &Buffer,
        target_start: Option<f64>,
        target_end: Option<f64>,
        source_start: Option<f64>,
        source_end: Option<f64>,
    ) -> i32;

    #[wasm_bindgen(method)]
    pub fn copy(
        this: &Buffer,
        target: &Buffer,
        target_start: Option<f64>,
        source_start: Option<f64>,
        source_end: Option<f64>,
    ) -> f64;

    #[wasm_bindgen(method)]
    pub fn entries(this: &Buffer) -> Iterator;

}
