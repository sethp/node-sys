use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type Global;

    pub static global: Global;
}
