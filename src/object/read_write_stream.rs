use crate::object::{ReadableStream, WriteableStream};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = ReadableStream, extends = WriteableStream)]
    pub type ReadWriteStream;
}
