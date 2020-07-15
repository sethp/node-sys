pub use crate::{
    class::{fs::*, Buffer},
    interface::{AppendFileOptions, MkdtempSyncOptions, ReadFileSyncOptions, WriteFileSyncOptions},
};
use js_sys::{Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "fs")]
extern "C" {
    pub fn access(path: &JsString, mode: Option<u32>, callback: &Function);

    #[wasm_bindgen(js_name = "accessSync")]
    pub fn access_sync(path: &JsString, mode: Option<u32>);

    #[wasm_bindgen(js_name = "appendFile")]
    pub fn append_file(
        path: &JsString,
        data: &Buffer,
        options: Option<AppendFileOptions>,
        callback: &Function,
    );

    // TODO: gets back a JsString if encoding is passed
    #[wasm_bindgen(js_name = "readFileSync", catch)]
    pub fn read_file_sync(
        path: &JsString,
        options: Option<ReadFileSyncOptions>,
    ) -> Result<Buffer, JsValue>;

    #[wasm_bindgen(js_name = "mkdtempSync")]
    pub fn mkdtemp_sync(prefix: &JsString, options: Option<MkdtempSyncOptions>) -> JsString;

    #[wasm_bindgen(js_name = "writeFileSync")]
    pub fn write_file_sync(file: &JsValue, data: &JsValue, options: Option<WriteFileSyncOptions>);
}
