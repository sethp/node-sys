use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReadFileSyncOptions {
    encoding: Option<JsString>,
    flag: Option<JsString>,
}

#[wasm_bindgen]
impl ReadFileSyncOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        encoding: Option<JsString>,
        flag: Option<JsString>,
    ) -> ReadFileSyncOptions {
        ReadFileSyncOptions { encoding, flag }
    }

    pub fn new() -> ReadFileSyncOptions {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn encoding(&self) -> Option<JsString> {
        self.encoding.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_encoding(&mut self, value: Option<JsString>) {
        self.encoding = value;
    }

    #[wasm_bindgen(getter)]
    pub fn flag(&self) -> Option<JsString> {
        self.flag.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_flag(&mut self, value: Option<JsString>) {
        self.flag = value;
    }
}
