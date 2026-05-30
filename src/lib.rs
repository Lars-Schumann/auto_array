#![no_std]
#[doc = include_str!("../README.md")]
#[macro_export]
macro_rules! auto_array {
    ($($(#[$attr:meta])* $vis:vis $const_or_static:ident $name:ident: [$ty:ty; _] = $array:expr;)*) => {
        $($(#[$attr])* $vis $const_or_static $name: [$ty; <[$ty]>::len(&$array)] = $array;)*
    };
}
