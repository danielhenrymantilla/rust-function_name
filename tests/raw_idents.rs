#[macro_use] extern crate function_name;

#[named]
fn r#if ()
{
    assert_eq!(function_name!(), "r#if");
}