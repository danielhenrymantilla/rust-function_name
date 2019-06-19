use ::function_name::function_name;

#[function_name]
fn my_super_duper_function ()
{
    dbg!(function_name!());
}

fn main ()
{
    my_super_duper_function();
}

