[Exposed=System]
interface Array: Object {
};

[Exposed=System]
interface ArrayBuffer: Object {
};

[Exposed=System]
interface Boolean: Object {
};

[Exposed=System]
interface DataView: Object {
};

[Exposed=System]
interface Date: Object {
};

[Exposed=System]
interface Error: Object {
  readonly attribute DOMString message;
  readonly attribute DOMString name;
};

[Exposed=System]
interface EvalError: Error {
};

[Exposed=System]
interface Function: Object {
};

[Exposed=System]
interface Generator: Object {
};

[Exposed=System]
interface Iterator {
};

[Exposed=System]
interface IteratorNext {
};

[Exposed=System]
interface Map: Object {
};

[Exposed=System]
interface Number: Object {
};

[Exposed=System]
interface Promise: Object {
};

[Exposed=System]
interface Proxy {
};

[Exposed=System]
interface RangeError: Error {
};

[Exposed=System]
interface ReferenceError: Error {
};

[Exposed=System]
interface RegExp: Object {
};

[Exposed=System]
interface Set: Object {
};

[Exposed=System]
interface SharedArrayBuffer: Object {
};

// [Exposed=System]
// interface String: Object {
// };

[Exposed=System]
interface Symbol {
};

[Exposed=System]
interface SyntaxError: Error {
};

[Exposed=System]
interface TypeError: Error {
};

[Exposed=System]
interface URIError: Error {
};

[Exposed=System]
interface WeakMap: Object {
};

[Exposed=System]
interface WeakSet: Object {
};
