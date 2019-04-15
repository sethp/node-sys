use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let msg = Array::new();
    msg.push(&"[examples/cwd]:".into());
    msg.push(&node_sys::process.cwd());
    msg.push(&"\n".into());
    web_sys::console::log(&msg);
    Ok(())
}
