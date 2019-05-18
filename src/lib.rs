pub mod macros;

#[cfg(feature = "assert")]
pub mod assert;

#[cfg(feature = "async_hooks")]
pub mod async_hooks;

#[cfg(feature = "buffer")]
pub mod buffer;

#[cfg(feature = "child_process")]
pub mod child_process;

#[cfg(feature = "cluster")]
pub mod cluster;

#[cfg(feature = "console")]
pub mod console;

#[cfg(feature = "constants")]
pub mod constants;

#[cfg(feature = "crypto")]
pub mod crypto;

#[cfg(feature = "dgram")]
pub mod dgram;

#[cfg(feature = "dns")]
pub mod dns;

#[cfg(feature = "domain")]
pub mod domain;

#[cfg(feature = "events")]
pub mod events;

#[cfg(feature = "fs")]
pub mod fs;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "http2")]
pub mod http2;

#[cfg(feature = "https")]
pub mod https;

#[cfg(feature = "inspector")]
pub mod inspector;

#[cfg(feature = "module")]
pub mod module;

#[cfg(feature = "net")]
pub mod net;

#[cfg(feature = "os")]
pub mod os;

#[cfg(feature = "path")]
pub mod path;

#[cfg(feature = "perf_hook")]
pub mod perf_hook;

#[cfg(feature = "process")]
pub mod process;

#[cfg(feature = "punycode")]
pub mod punycode;

#[cfg(feature = "querystring")]
pub mod querystring;

#[cfg(feature = "readline")]
pub mod readline;

#[cfg(feature = "repl")]
pub mod repl;

#[cfg(feature = "stream")]
pub mod stream;

#[cfg(feature = "string_decoder")]
pub mod string_decoder;

#[cfg(feature = "timers")]
pub mod timers;

#[cfg(feature = "tls")]
pub mod tls;

#[cfg(feature = "trace_events")]
pub mod trace_events;

#[cfg(feature = "tty")]
pub mod tty;

#[cfg(feature = "url")]
pub mod url;

#[cfg(feature = "util")]
pub mod util;

#[cfg(feature = "v8")]
pub mod v8;

#[cfg(feature = "vm")]
pub mod vm;

#[cfg(feature = "worker_threads")]
pub mod worker_threads;

#[cfg(feature = "zlib")]
pub mod zlib;

// FIXME
pub trait Cast {
    fn cast<T>(&self) -> T
    where
        Self: Clone + Into<T>,
    {
        Into::<T>::into(self.clone())
    }
}
impl<A> Cast for A {}
