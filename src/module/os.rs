use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "os")]
extern {

    pub fn arch() -> JsString;

    pub fn cpus() -> Box<[JsValue]>;

    pub fn endianness() -> JsString;

    pub fn freemem() -> f64;

    #[wasm_bindgen(js_name = "getPriority")]
    pub fn get_priority(pid: Option<u32>) -> u32;

    pub fn homedir() -> JsString;

    pub fn hostname() -> JsString;

    pub fn loadavg() -> Box<[JsValue]>;

    #[wasm_bindgen(js_name = "networkInterfaces")]
    pub fn network_interfaces() -> Object;

    pub fn platform() -> JsString;
    pub fn tmpdir() -> JsString;
}
