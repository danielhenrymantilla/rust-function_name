#![cfg_attr(feature = "better-docs",
    cfg_attr(all(), doc = include_str!("../README.md")),
)]
#![doc(test(attr(deny(warnings), allow(unused),)))]
//!
#![warn(missing_docs)]
#![no_std]

/// Entry point of the crate.
///
/** ```rust
use ::function_name::named;

#[test]
#[named]
fn foo ()
{
    assert_eq!(function_name!(), "foo");
}
``` */
pub use ::function_name_proc_macro::named;
