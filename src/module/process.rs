use crate::module::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type Process;

    pub static process: Process;
}

