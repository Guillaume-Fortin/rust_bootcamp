// Fix the code to make it compile.

fn main() {
    let adder1 = get_adder(-2);
    let adder2 = get_adder(100);
    assert_eq!(adder1(20), 18);
    assert_eq!(adder2(10), 110);
}

fn get_adder(to_add: i32) -> impl Fn(i32) -> i32 {
    move |x| x + to_add
}
