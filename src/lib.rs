//! Raw bindings to the Node.js API for projects using wasm-bindgen

#![deny(clippy::all)]
// #![deny(missing_docs)] // FIXME: wasm-bindgen macros break this

pub(crate) mod module;
pub(crate) mod object;
pub use module::*;
pub use object::*;
