// Define the `double` closure & make the code execute successfully.

fn main() {
    let double = |value: i32| value * 2;
    assert_eq!(double(5), 10);
    assert_eq!(double(-10), -20);
}
