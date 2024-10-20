// Everything seems correct, but the code does not compile. Maybe it has to do with the position of defining a macro.

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
