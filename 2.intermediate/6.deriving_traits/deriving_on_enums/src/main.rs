// Only one trait needs to be derived. Can you figure out which?

#[derive(PartialEq)]
enum Size {
    Small,
    Medium,
    Large,
}

fn main() {
    let my_size = Size::Small;
    if my_size == Size::Small {
        println!("I can fit in any size!");
    }
}
