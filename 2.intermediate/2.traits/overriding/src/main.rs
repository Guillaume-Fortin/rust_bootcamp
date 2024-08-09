// Make the code execute successfully by correctly implementing Message trait for Cat type.

trait Message {
    fn message(&self) -> String {
        "Default Message!".to_string()
    }
}

struct Fish;
struct Cat;

impl Message for Fish {}

impl Message for Cat {
    fn message(&self) -> String {
        "Meow 🐱".to_string()
    }
}

fn main() {
    let fish = Fish;
    let cat = Cat;
    assert_eq!(String::from("Default Message!"), fish.message());
    assert_eq!(String::from("Meow 🐱"), cat.message());
}
