pub use crate::{class::Buffer, interface::TranscodeEncoding};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn transcode(source: &Uint8Array, from_enc: &TranscodeEncoding, to_enc: &TranscodeEncoding) -> Buffer;
}
