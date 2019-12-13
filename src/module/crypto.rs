use crate::{class::crypto::Hash, interface::StreamTransformOptions};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern {
    #[wasm_bindgen(js_name = "createHash")]
    pub fn create_hash(algorithm: &JsString, options: Option<StreamTransformOptions>) -> Hash;
}
