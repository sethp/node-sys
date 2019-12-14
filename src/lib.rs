//! Raw bindings to the Node.js API for projects using wasm-bindgen

#![deny(clippy::all)]
// #![deny(missing_docs)] // FIXME: wasm-bindgen macros break this

pub(crate) mod class;
pub(crate) mod globals;
pub(crate) mod interface;
pub(crate) mod module;

pub use class::{Buffer, EventEmitter, Immediate, Module, Timeout, Wasi};
pub use globals::*;
pub use interface::*;
pub use module::*;
