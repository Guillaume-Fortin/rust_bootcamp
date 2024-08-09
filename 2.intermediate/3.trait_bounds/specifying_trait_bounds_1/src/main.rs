// Complete the function signatures.

trait Message {
    fn message(&self) -> String {
        "I love Rust 🦀".to_string()
    }
}

fn print_msg1<T>(input: &T)
where
    T: Message,
{
    println!("{}", input.message());
}

fn print_msg2(input: &impl Message) {
    println!("{}", input.message());
}

fn print_msg3<T>(input: &T)
where
    T: Message,
{
    println!("{}", input.message());
}

struct Dummy;

impl Message for Dummy {}

fn main() {
    let var = Dummy;
    print_msg1(&var);
    print_msg2(&var);
    print_msg3(&var);
}
