// Fix the code by bringing `my_macros` in scope (You have to mark `macros` module with something).
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
