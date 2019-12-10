use crate::interface::NodeRequireFunction;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type NodeModule;

    #[wasm_bindgen(method, getter)]
    pub fn children(this: &NodeModule) -> Box<[JsValue]>;

    #[wasm_bindgen(method, getter)]
    pub fn exports(this: &NodeModule) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn filename(this: &NodeModule) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &NodeModule) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn loaded(this: &NodeModule) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn parent(this: &NodeModule) -> Option<NodeModule>;

    #[wasm_bindgen(method, getter)]
    pub fn paths(this: &NodeModule) -> Box<[JsValue]>;

    #[wasm_bindgen(method, getter)]
    pub fn require(this: &NodeModule) -> NodeRequireFunction;
}
