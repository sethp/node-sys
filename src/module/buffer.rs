pub use crate::object::{Buffer, TranscodeEncoding};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type BufferModule;

    pub static buffer: BufferModule;

    pub fn transcode(
        this: &BufferModule,
        source: &Uint8Array,
        from_enc: &TranscodeEncoding,
        to_enc: &TranscodeEncoding,
    ) -> Buffer;
}
