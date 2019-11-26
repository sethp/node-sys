/// The assert module provides a set of assertion functions for verifying invariants.
pub mod assert;

/// The async_hooks module provides an API to register callbacks tracking the lifetime of
/// asynchronous resources created inside a Node.js application.
pub mod async_hooks;

pub mod buffer;

/// The child_process module provides the ability to spawn child processes in a manner that is
/// similar, but not identical, to popen(3).
pub mod child_process;

/// The cluster module allows easy creation of child processes that all share server ports.
pub mod cluster;

/// The console module provides a simple debugging console that is similar to the JavaScript console
/// mechanism provided by web browsers.
pub mod console;

/// The crypto module provides cryptographic functionality that includes a set of wrappers for
/// OpenSSL's hash, HMAC, cipher, decipher, sign, and verify functions.
pub mod crypto;

/// The dgram module provides an implementation of UDP Datagram sockets.
pub mod dgram;

/// The dns module contains functions for performing name resolution either through the OS or by
/// connecting to a name server.
pub mod dns;

/// The events module provides functionality for working with `EventEmitter`.
pub mod events;

/// The fs module provides an API for interacting with the file system in a manner closely modeled
/// around standard POSIX functions.
pub mod fs;

pub(crate) mod global;

/// The http module provides an implementation of the HTTP protocol for clients and servers.
pub mod http;

/// The http/2 module provides an implementation of the HTTP/2 protocol for clients and servers.
pub mod http2;

/// The https module provides an implementation of HTTP over TLS/SSL for clients and servers
pub mod https;

/// The inspector module provides an API for interacting with the V8 inspector.
pub mod inspector;

/// The net module provides an asynchronous network API for creating stream-based TCP or IPC servers
/// (net.createServer()) and clients (net.createConnection()).
pub mod net;

/// The os module provides operating system-related utility methods and properties.
pub mod os;

/// The path module provides utilities for working with file and directory paths.
pub mod path;

/// The perf_hook module provides an implementation of the W3C Performance Timeline specification.
pub mod perf_hooks;

/// The process module provides information about, and control over, the current Node.js process.
pub(crate) mod process;

/// The querystring module provides utilities for parsing and formatting URL query strings.
pub mod querystring;

/// The readline module provides an interface for reading data from a Readable stream (such as
/// process.stdin) one line at a time.
pub mod readline;

/// The repl module provides a Read-Eval-Print-Loop (REPL) implementation that is available both as
/// a standalone program or includible in other applications.
pub mod repl;

pub(crate) mod require;

/// The stream module provides an API for implementing the stream interface.
pub mod stream;

/// The string_decoder module provides an API for decoding Buffer objects into strings in a manner
/// that preserves encoded multi-byte UTF-8 and UTF-16 characters.
pub mod string_decoder;

/// The timer module exposes a global API for scheduling functions to be called at some future
/// period of time.
pub mod timers;

/// The tls module provides an implementation of the Transport Layer Security (TLS) and Secure
/// Socket Layer (SSL) protocols that is built on top of OpenSSL.
pub mod tls;

/// The trace_event module provides functionality to centralize tracing information generated by V8,
/// Node.js core, and userspace code.
pub mod trace_events;

/// The tty module provides the tty.ReadStream and tty.WriteStream classes.
pub mod tty;

/// The url module provides utilities for URL resolution and parsing.
pub mod url;

/// The util module is primarily designed to support the needs of Node.js' own internal APIs.
pub mod util;

/// The v8 module exposes APIs that are specific to the version of V8 built into the Node.js binary.
pub mod v8;

/// The vm module provides APIs for compiling and running code within V8 Virtual Machine contexts.
pub mod vm;

/// The worker_threads module enables the use of threads that execute JavaScript in parallel.
pub mod worker_threads;

/// The zlib module provides compression functionality implemented using Gzip and Deflate/Inflate,
/// as well as Brotli.
pub mod zlib;

pub use global::global;
pub use process::process;
pub use require::require;
