// Make the code compile by defining Operation enum with a single generic type.
enum Operation<T> {
    Add(T, T),
    Mul(T, T),
    Sub { left: T, right: T },
    Div { dividend: T, divisor: T },
}

fn main() {
    let _op1 = Operation::Add(15u8, 10u8);
    let _op2 = Operation::Mul(150, 23);
    let _op3 = Operation::Sub {
        left: 120,
        right: 50,
    };
    let _op4 = Operation::Div {
        dividend: 10.23,
        divisor: 2.43,
    };
}
