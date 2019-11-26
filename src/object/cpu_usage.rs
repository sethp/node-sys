use js_sys::Number;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type CpuUsage;

    #[wasm_bindgen(method)]
    pub fn user(this: &CpuUsage) -> Number;

    #[wasm_bindgen(method)]
    pub fn system(this: &CpuUsage) -> Number;
}
