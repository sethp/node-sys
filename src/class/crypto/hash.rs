use crate::class::{buffer::Buffer, stream::Transform};
use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern {
    #[wasm_bindgen(extends = Transform)]
    #[derive(Clone, Debug)]
    pub type Hash;

    #[wasm_bindgen(method)]
    pub fn digest(this: &Hash) -> Buffer;

    #[wasm_bindgen(method, js_name = "digest")]
    pub fn digest_with_encoding(this: &Hash, encoding: &JsString) -> JsString;

    #[wasm_bindgen(method)]
    pub fn update(this: &Hash, data: &Object);

    #[wasm_bindgen(method, js_name = "update")]
    pub fn update_with_encoding(this: &Hash, data: &JsString, encoding: Option<&JsString>) -> Hash;
}
