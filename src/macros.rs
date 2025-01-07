/// Construct a JsValue using a js-like syntax
#[macro_export]
macro_rules! jsobj {
    // Base case: no key-value pairs
    () => {
        web_sys::js_sys::Object::new()
    };

    // Match a key-value pair where the value is a nested object
    (
        $key:literal: { $($inner_key:literal: $inner_val:tt),* $(,)? }, $($rest:tt)*
    ) => {{
        let obj = jsobj!($($rest)*);
        let nested_obj = jsobj! { $($inner_key: $inner_val),* , };
        web_sys::js_sys::Reflect::set(&obj, &$key.into(), &nested_obj).unwrap();
        obj
    }};

    // Match a key-value pair where the value is an expression
    (
        $key:literal: $val:expr, $($rest:tt)*
    ) => {{
        let obj = jsobj!($($rest)*);
        web_sys::js_sys::Reflect::set(&obj, &$key.into(), &$val.into()).unwrap();
        obj
    }};

    // Handle trailing commas
    (
        $($key:literal: $val:tt),* ,
    ) => {
        jsobj!($($key: $val),*)
    };

    // Handle multiple key-value pairs
    (
        $key:literal: $val:tt, $($rest:tt)*
    ) => {{
        let obj = jsobj!($($rest)*);
        web_sys::js_sys::Reflect::set(&obj, &$key.into(), &jsobj!($val)).unwrap();
        obj
    }};
}

/// Recursively apply Reflect::get
#[macro_export]
macro_rules! jsgets {
    ($parent:expr, $child:expr $(,$children:expr)*) => {{
        let parent = web_sys::js_sys::Reflect::get(&$parent, &$child.into())
            .expect(format!("Failed to get js property: {}", $child).as_str());
        web_sys::console::log_1(&parent);

        vec![$($children),*].into_iter().fold(parent, |parent, child: &str|{
            let parent = web_sys::js_sys::Reflect::get(&parent, &child.into())
            .expect(format!("Failed to get js property: {}", child).as_str());
            web_sys::console::log_2(&"jsget".into(), &parent);
            parent
        })
    }}
}

#[cfg(test)]
mod test {
    use gloo::utils::window;

    #[test]
    fn test_jsobj_simple() {
        let obj = jsobj! {
            "hello" : "world",
            "bing" : 8,
        };
        println!("{:?}", obj);
    }

    #[test]
    fn test_jsobj_complex() {
        let x = 5;

        let obj = jsobj! {
            "hello": "world",
            "bing": 8,
            "bing2": 8 + 5,
            "bong": {
                "gyatt": "bap",
                "gyatt1": {
                    "skrrt": (-6),
                    "skrrt2": x,
                }
            },
            "bong": {  },
        };
        println!("{:?}", obj);
    }

    #[test]
    fn test_jsgets() {
        let window = window();
        let marker = jsgets!(window, "google");
        let marker = jsgets!(window, "google", "maps", "marker");
    }
}
