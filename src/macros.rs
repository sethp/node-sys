#[macro_export]
macro_rules! options {
    ($($rest:tt)*) => {{
        let this = js_sys::Object::new();
        $crate::__options_rest!(this | , $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __options_rest {
    { $this:ident | , $key:ident $($rest:tt)* } => {
        {
            $crate::__options_key_val!($this | (stringify!($key)) $($rest)*)
        }
    };
    { $this:ident | , $key:tt $($rest:tt)* } => {
        {
            $crate::__options_key_val!($this | $key $($rest)*)
        }
    };
    { $this:ident | $(,)? } => {
        {
            $this.into()
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __options_key_val {
    { $this:ident | $key:tt : $val:tt $($rest:tt)* } => {{
        js_sys::Reflect::set(&$this, &$key.into(), &$val.into()).unwrap();
        $crate::__options_rest!($this | $($rest)*)
    }};
}
