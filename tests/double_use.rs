#[macro_use]
extern crate function_name;

#[named]
fn foo() {
    assert_eq!(function_name!(), "foo");
}

#[named]
fn bar() {
    assert_eq!(function_name!(), "bar");
}

#[test]
fn main() {
    foo();
    bar();
}
