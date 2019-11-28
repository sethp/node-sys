pub use crate::class::EventEmitter;

use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "events")]
extern {
    /// Creates a `Promise` that is fulfilled when the `EventEmitter` emits the given event or that
    /// is rejected when the `EventEmitter` emits `"error"`. The `Promise` will resolve with an
    /// array of all the arguments emitted to the given event.
    pub fn once(emitter: &EventEmitter, name: &str) -> Promise;
}
