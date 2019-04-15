use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    web_sys::console::log_1(&JsString::concat(
        &"[examples/cwd]: ".into(),
        &node_sys::process.cwd(),
    ));
    Ok(())
}
