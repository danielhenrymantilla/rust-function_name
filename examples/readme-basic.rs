use ::function_name::named;

#[named]
fn my_super_duper_function() {
    dbg!(function_name!());
}

fn main() {
    my_super_duper_function();
}
