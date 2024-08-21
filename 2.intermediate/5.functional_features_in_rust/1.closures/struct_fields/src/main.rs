// Complete the struct definition & the implementation block.

struct BinaryOperation<T, U>
where
    T: Copy,
    U: Fn(T, T) -> T,
{
    operand1: T,
    operand2: T,
    operation: U,
}

impl<T, U> BinaryOperation<T, U>
where
    T: Copy,
    U: Fn(T, T) -> T,
{
    fn calculate(&self) -> T {
        (self.operation)(self.operand1, self.operand2)
    }
}

fn main() {
    let multiply = BinaryOperation {
        operand1: 20,
        operand2: 6,
        operation: |a, b| a * b,
    };
    let divide = BinaryOperation {
        operand1: 22.0,
        operand2: 7.0,
        operation: |x, y| x / y,
    };
    println!(
        "{} x {} = {}",
        multiply.operand1,
        multiply.operand2,
        multiply.calculate()
    );
    println!(
        "{} / {} = {}",
        divide.operand1,
        divide.operand2,
        divide.calculate()
    );
}
