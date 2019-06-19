# `#[function_name]`

Function attribute that generates a `function_name!` macro
in the scope of the function's body.

The generated `function_name!()` is a macro that expands to
the name of the annotated function, as a string literal.

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

