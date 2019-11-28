use crate::class::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type ReadableStream;
}
