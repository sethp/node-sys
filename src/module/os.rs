use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "os")]
extern {

    pub fn arch() -> JsString;

    pub fn cpus() -> Box<[JsValue]>;

    #[wasm_bindgen(js_name = "networkInterfaces")]
    pub fn network_interfaces() -> Object;
    pub fn tmpdir() -> JsString;
}
