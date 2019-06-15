callback StartStackFn = any (any... arguments);

dictionary AssertionErrorOptions {
    any actual;
    any expected;
    DOMString? message;
    DOMString? operator;
    StartStackFn? startStackFn;
};

[Exposed=System, Constructor(optional AssertionErrorOptions options)]
interface AssertionError: Error {
    readonly attribute any actual;
    readonly attribute DOMString code;
    readonly attribute any expected;
    readonly attribute boolean generatedMessage;
    readonly attribute DOMString message;
    readonly attribute DOMString name;
    readonly attribute DOMString operator;
};

namespace assert {
    [Throws]
    void strictEqual(any actual, any expected, (DOMString or Error)? message);
};
