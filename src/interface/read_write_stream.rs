use crate::interface::{ReadableStream, WritableStream};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = ReadableStream, extends = WritableStream)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type ReadWriteStream;
}
