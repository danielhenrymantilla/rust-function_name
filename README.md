## `#[function_name]`

Function attribute that generates a `function_name!` macro
in the scope of the function's body.

The generated `function_name!()` is a macro that expands to
the name of the annotated function, as a string literal.

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)][Repository] [![Latest version](https://img.shields.io/crates/v/function_name.svg)][crates.io] [![Documentation](https://docs.rs/function_name/badge.svg)][Documentation] [![License](https://img.shields.io/crates/l/function_name.svg)](https://github.com/danielhenrymantilla/rust-function_name#license)

## Examples

```rust
use ::function_name::function_name;

#[function_name]
fn my_super_duper_function ()
{
    assert_eq!(
        function_name!(),
        "my_super_duper_function",
    );
}
```

Since the generated `function_name!` expands to a string literal,
it can be used with other macros such as [`concat!`](
https://doc.rust-lang.org/std/macro.concat.html):


```rust
#[macro_use] extern crate function_name;

macro_rules! function_path {() => (concat!(
    module_path!(), "::", function_name!()
))}

pub mod foo {
    pub mod bar {
        #[function_name]
        pub fn baz ()
        {
            assert_eq!(
                function_path!(),
                [
                    env!("CARGO_PKG_NAME"),
                    "foo", "bar",
                    "baz",
                ].join("::"),
            );
        }
    }
}
```

[Repository]: https://github.com/danielhenrymantilla/rust-function_name
[Documentation]: https://docs.rs/function_name
[crates.io]: https://crates.io/crates/function_name

