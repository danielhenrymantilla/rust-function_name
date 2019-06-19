#[macro_use] extern crate function_name;

macro_rules! function_path {() => (concat!(
    module_path!(), "::", function_name!()
))}

pub mod foo {
    pub mod bar {
        #[function_name]
        pub fn baz ()
        {
            dbg!(function_path!());
        }
    }
}

fn main ()
{
    foo::bar::baz();
}

