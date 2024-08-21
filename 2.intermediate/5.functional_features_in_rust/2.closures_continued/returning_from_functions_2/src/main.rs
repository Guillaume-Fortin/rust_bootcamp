// Fix the code to make it compile.

enum Operation {
    Add,
    Sub,
    Mul,
}

fn get_implementation(operation: Operation, operand2: i32) -> Box<dyn Fn(i32) -> i32> {
    match operation {
        Operation::Add => Box::new(move |x| x + operand2),
        Operation::Sub => Box::new(move |x| x - operand2),
        Operation::Mul => Box::new(move |x| x * operand2),
    }
}

fn main() {
    const OPERAND2: i32 = 5;
    let adder = get_implementation(Operation::Add, OPERAND2);
    let multiplier = get_implementation(Operation::Mul, OPERAND2);
    assert_eq!(adder(10), 15);
    assert_eq!(multiplier(10), 50);
}
