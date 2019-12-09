use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "fs")]
extern {
    #[wasm_bindgen(extends = Object, js_name = "FSWatcher")]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type FsWatcher;

    // FIXME: event overloads
}
