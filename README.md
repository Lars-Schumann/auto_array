## auto_array
A declarative macro to automatically calculate the length of arrays.\
This is useful e.g. for arrays whose length could be changed by conditional compilation.

## Example
```rust
use auto_array::auto_array;
fn main() {
    auto_array! {
        // Additional attributes, docs, and visibility are supported.
        /// An unused const array.
        #[allow(unused)]
        const A: [u8; _] = [3, 3, 3];
        /// A static array whose length depends on conditional compilation.
        pub(crate) static B: [u8; _] = [1, #[cfg(unix)] 2];
    }
    assert_eq!(A, [3, 3, 3]);
    assert_eq!(B, [1, #[cfg(unix)] 2]);
}
```
