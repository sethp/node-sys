#[macro_export]
macro_rules! object {
    ($($rest:tt)*) => {{
        let this = js_sys::Object::new();
        $crate::__object_rest!(this | , $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __object_rest {
    // try to parse the key as an identifier (unquoted) first
    { $this:ident | , $key:ident $($rest:tt)* } => {
        {
            $crate::__object_key_val!($this | (stringify!($key)) $($rest)*)
        }
    };
    // try to parse the key as a token tree if it isn't an identifier
    { $this:ident | , $key:tt $($rest:tt)* } => {
        {
            $crate::__object_key_val!($this | $key $($rest)*)
        }
    };
    // when all fields are processed, return the object
    { $this:ident | $(,)? } => {
        {
            $this.into()
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __object_key_val {
    { $this:ident | $key:tt : $val:tt $($rest:tt)* } => {{
        js_sys::Reflect::set(&$this, &$key.into(), &$val.into()).unwrap();
        $crate::__object_rest!($this | $($rest)*)
    }};
}
