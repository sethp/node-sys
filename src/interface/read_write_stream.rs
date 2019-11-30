use crate::interface::{ReadableStream, WritableStream};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = ReadableStream, extends = WritableStream)]
    pub type ReadWriteStream;
}
