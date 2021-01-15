#[allow(dead_code)]
use std::collections::HashMap;

#[macro_export]
macro_rules! say_hello {
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}
#[macro_export]
macro_rules! mapping{
    ($($key:expr => $val:expr),+) => {
        let mut map = HashMap::new();
        $(
            map.insert($key,$val);
        )+
    map
    }
}
