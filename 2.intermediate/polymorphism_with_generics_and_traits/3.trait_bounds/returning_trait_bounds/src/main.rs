// Complete function signature of `get_bigger`.
// Only Addable trait's functions should be callable on its return value.

trait Addable {
    fn add_one(&self) -> Self;
    fn are_equal(a: &Self, b: &Self) -> bool;
}

impl Addable for u8 {
    fn add_one(&self) -> Self {
        if *self == u8::MAX {
            *self
        } else {
            self + 1
        }
    }
    fn are_equal(a: &Self, b: &Self) -> bool {
        a == b
    }
}

fn get_bigger(a: u8, b: u8) -> impl Addable {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let (num1, num2) = (125, 220);
    let bigger = get_bigger(num1, num2);
    if Addable::are_equal(&bigger, &bigger.add_one()) {
        println!("Bigger number has max value")
    } else {
        println!("Both numbers are smaller than max value");
    }
}
