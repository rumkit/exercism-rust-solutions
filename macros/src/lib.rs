#[macro_export]
macro_rules! hashmap {
    ($( $key: literal => $val: expr ),+$(,)?) => {{
        let mut hm = ::std::collections::HashMap::new();
        $(
            hm.insert($key, $val);
        )+
        hm
    }};
    () => {
        ::std::collections::HashMap::new()
    };
}

/// This module contains doctests, which allows writing tests where a code
/// snippet is supposed to fail to compile. These tests also have "ignore"
/// attributes, makes sure to remove them when solving this exercise locally.
pub mod compile_fail_tests;
