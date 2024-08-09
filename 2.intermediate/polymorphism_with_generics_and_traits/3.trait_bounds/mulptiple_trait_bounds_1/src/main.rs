// Make the code compile by completing the function signature of `get_double_str`.

trait Double {
    fn double(&self) -> Self;
}

trait Printable {
    fn convert_to_str(self) -> String;
}

fn get_double_str(input: impl Double + Printable) -> String {
    let doubled = input.double();
    doubled.convert_to_str()
}

impl Double for i32 {
    fn double(&self) -> Self {
        2 * self
    }
}

impl Printable for i32 {
    fn convert_to_str(self) -> String {
        format!("{self}")
    }
}

fn main() {
    let num = 22;
    let mut msg = format!("{num} doubled is ");
    msg.push_str(&get_double_str(num));
    println!("{msg}");
}
