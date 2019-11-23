use js_sys::{Function, Object, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "events")]
extern {
    /// An object that emits events which trigger functions registered as listeners.
    #[wasm_bindgen(extends = Object)]
    pub type EventEmitter;

    #[wasm_bindgen(constructor)]
    pub fn new() -> EventEmitter;

    /// By default, a maximum of `10` listeners can be registered for any single event. This limit
    /// can be changed for individual `EventEmitter` instances using the
    /// `emitter::set_max_listeners` method. To change the default for all `EventEmitter`
    /// instances, the `EventEmitter::default_max_listeners` property can be used.
    #[wasm_bindgen(method, getter, js_name = "defaultMaxListeners")]
    pub fn default_max_listeners(this: &EventEmitter) -> usize;

    #[wasm_bindgen(method, setter, js_name = "defaultMaxListeners")]
    pub fn set_default_max_listeners(this: &EventEmitter, default_max_listeners: usize);

    /// Adds the listener function to the end of the `listener` array for the event named
    /// `event_name`.
    #[wasm_bindgen(method, js_name = "addListener")]
    pub fn add_listener(this: &EventEmitter, event_name: &str, listener: &Function) -> EventEmitter;

    /// Synchronously calls each of the listeners registered for the event named `event_name`, in
    /// the order they were registered, passing the supplied arguments to each.
    #[wasm_bindgen(method, variadic)]
    pub fn emit(this: &EventEmitter, event_name: &str, args: Box<[JsValue]>) -> bool;

    /// Returns an array listing the events for which the emitter has registered listeners. The
    /// values in the array will be strings or Symbols.
    #[wasm_bindgen(method, js_name = "eventNames")]
    pub fn event_names(this: &EventEmitter) -> Box<[JsValue]>;

    /// Returns the current max listener value for the EventEmitter which is either set by
    /// `EventEmitter::setMaxListeners` or defaults to `EventEmitter::defaultMaxListeners`.
    #[wasm_bindgen(method, js_name = "getMaxListeners")]
    pub fn get_max_listeners(this: &EventEmitter);

    /// Returns the number of listeners listening to the event named `event_name`.
    #[wasm_bindgen(method, js_name = "listenerCount")]
    pub fn listener_count(this: &EventEmitter) -> usize;

    /// Returns a copy of the array of listeners for the event named `event_name`.
    #[wasm_bindgen(method)]
    pub fn listeners(this: &EventEmitter, event_name: &str) -> Box<[JsValue]>; // FIXME: Box<[Function]>

    /// Alias for `EventEmitter::remove_listener`.
    #[wasm_bindgen(method)]
    pub fn off(this: &EventEmitter, event_name: &str, listener: &Function) -> EventEmitter;

    /// Adds the listener function to the end of the `listener` array for the event named
    /// `event_name`.
    #[wasm_bindgen(method, js_name = "on")]
    pub fn on(this: &EventEmitter, event_name: &str, listener: &Function) -> EventEmitter;

    /// Adds a one-time `listener` function for the event named `event_name`. The next time
    /// `event_name` is triggered, this listener is removed and then invoked.
    #[wasm_bindgen(method, js_name = "once")]
    pub fn once(this: &EventEmitter, event_name: &str, listener: &Function) -> EventEmitter;

    /// Adds the `listener` function to the beginning of the listeners array for the event named
    /// `event_name`.
    #[wasm_bindgen(method, js_name = "prependListener")]
    pub fn prepend_listener(this: &EventEmitter, event_name: &str, listener: &Function) -> EventEmitter;

    /// Adds a one-time `listener` function for the event named `event_name` to the beginning of the
    /// listeners array.
    #[wasm_bindgen(method, js_name = "prependOnceListener")]
    pub fn prepend_once_listener(this: &EventEmitter, event_name: &str, listener: &Function) -> EventEmitter;

    /// Removes all listeners, or those of the specified `event_name`.
    #[wasm_bindgen(method, js_name = "removeAllListeners")]
    pub fn remove_all_listeners(this: &EventEmitter, event_name: Option<&str>) -> EventEmitter;

    /// Removes the specified `listener` from the listener array for the event named `event_name`.
    #[wasm_bindgen(method, js_name = "removeListener")]
    pub fn remove_listener(this: &EventEmitter, event_name: &str, listener: &Function) -> EventEmitter;

    /// By default `EventEmitters` will print a warning if more than `10` listeners are added for a
    /// particular event.
    #[wasm_bindgen(method, js_name = "setMaxListeners")]
    pub fn set_max_listeners(this: &EventEmitter, n: usize) -> EventEmitter;

    /// Returns a copy of the array of listeners for the event named `event_name`, including any
    /// wrappers (such as those created by `.once()`).
    #[wasm_bindgen(method, js_name = "rawListeners")]
    pub fn raw_listeners(this: &EventEmitter, event_name: &str) -> Box<[JsValue]>;

    /// Creates a `Promise` that is fulfilled when the `EventEmitter` emits the given event or that
    /// is rejected when the `EventEmitter` emits `"error"`. The `Promise` will resolve with an
    /// array of all the arguments emitted to the given event.
    #[wasm_bindgen]
    pub fn once(event_emitter: &EventEmitter, name: &str) -> Promise;
}
