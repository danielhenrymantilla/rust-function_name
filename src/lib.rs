
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(unused_variables))))]

#![warn(missing_docs)]

#![doc(html_root_url = "https://docs.rs/function_name")]

#![cfg_attr(feature = "nightly",
    feature(external_doc)
)]
#![cfg_attr(feature = "nightly",
    doc(include = "../README.md")
)]
#![cfg_attr(not(feature = "nightly"),
    doc = "See [crates.io](https://crates.io/crates/function_name)"
)]
#![cfg_attr(not(feature = "nightly"),
    doc = "for more info about this crate."
)]

#[doc(inline)]
pub use ::function_name_proc_macro::function_name;

#[doc(hidden)]
pub use ::function_name_proc_macro::function_name_hack;

#[doc(hidden)]
pub mod function_name { pub use super::function_name_hack; }

