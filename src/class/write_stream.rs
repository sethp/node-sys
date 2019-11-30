use crate::interface::WritableStream;
use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen(module = "events")]
extern {
    #[wasm_bindgen(extends = WritableStream)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type WriteStream;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn close(this: &WriteStream);

    #[wasm_bindgen(method, js_name = "bytesWritten")]
    pub fn bytes_written(this: &WriteStream) -> usize;

    #[wasm_bindgen(method)]
    pub fn path(this: &WriteStream) -> JsValue; // Buffer | string
}

//******************************//
// Instance Methods (Overloads) //
//******************************//

#[wasm_bindgen]
impl WriteStream {
    pub fn add_listener_with_open(&self, listener: &Function) -> WriteStream {
        self.add_listener("open", listener).unchecked_into()
    }

    pub fn add_listener_with_close(&self, listener: &Function) -> WriteStream {
        self.add_listener("close", listener).unchecked_into()
    }

    pub fn on_with_open(&self, listener: &Function) -> WriteStream {
        self.on("open", listener).unchecked_into()
    }

    pub fn on_with_close(&self, listener: &Function) -> WriteStream {
        self.on("close", listener).unchecked_into()
    }

    pub fn once_with_open(&self, listener: &Function) -> WriteStream {
        self.once("open", listener).unchecked_into()
    }

    pub fn once_with_close(&self, listener: &Function) -> WriteStream {
        self.once("close", listener).unchecked_into()
    }

    pub fn prepend_listener_with_open(&self, listener: &Function) -> WriteStream {
        self.prepend_listener("open", listener).unchecked_into()
    }

    pub fn prepend_listener_with_close(&self, listener: &Function) -> WriteStream {
        self.prepend_listener("close", listener).unchecked_into()
    }

    pub fn prepend_once_listener_with_open(&self, listener: &Function) -> WriteStream {
        self.prepend_once_listener("open", listener).unchecked_into()
    }

    pub fn prepend_once_listener_with_close(&self, listener: &Function) -> WriteStream {
        self.prepend_once_listener("close", listener).unchecked_into()
    }
}
