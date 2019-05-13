#[macro_export]
macro_rules! options {
    ($($rest:tt)*) => {{
        use js_sys::{Object, Reflect};
        let this = Object::new();
        $crate::__options_rest!(this , $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __options_rest {
    { $this:tt , $key:ident $($rest:tt)* } => {
        {
            $crate::__options_key_val!($this , (stringify!($key)) $($rest)*)
        }
    };
    { $this:tt , $key:tt $($rest:tt)* } => {
        {
            $crate::__options_key_val!($this , $key $($rest)*)
        }
    };
    { $this:tt , } => {
        {
            $this.into()
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __options_key_val {
    { $this:tt , $key:tt : $val:tt , $($rest:tt)* } => {{
        Reflect::set(&$this, &$key.into(), &$val.into()).unwrap();
        $crate::__options_rest!($this , $($rest)*)
    }};
}
