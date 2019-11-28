pub use crate::class::EventEmitter;

use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "events")]
extern {
    pub type Events;

    pub static events: Events;

    /// Creates a `Promise` that is fulfilled when the `EventEmitter` emits the given event or that
    /// is rejected when the `EventEmitter` emits `"error"`. The `Promise` will resolve with an
    /// array of all the arguments emitted to the given event.
    #[wasm_bindgen]
    pub fn once(event_emitter: &Events, emitter: &EventEmitter, name: &str) -> Promise;
}
