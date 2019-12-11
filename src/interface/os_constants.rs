use crate::interface::{OsConstantsDlopen, OsConstantsErrno, OsConstantsLibuv, OsConstantsPriority, OsConstantsSignal};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type OsConstants;

    #[wasm_bindgen(method, getter)]
    pub fn dlopen(this: &OsConstants) -> OsConstantsDlopen;

    #[wasm_bindgen(method, getter)]
    pub fn errno(this: &OsConstants) -> OsConstantsErrno;

    #[wasm_bindgen(method, getter)]
    pub fn libuv(this: &OsConstants) -> OsConstantsLibuv;

    #[wasm_bindgen(method, getter)]
    pub fn priority(this: &OsConstants) -> OsConstantsPriority;

    #[wasm_bindgen(method, getter)]
    pub fn signal(this: &OsConstants) -> OsConstantsSignal;
}
