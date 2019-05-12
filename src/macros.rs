#[macro_export]
macro_rules! options {
    ($($key:tt : $val:expr $(,)?),*) => {
        {
            let obj = Object::new();
            { $(Reflect::set(&obj, &$key.into(), &$val.into()).unwrap());* };
            obj.into()
        }
    };
}
