// Below is the classic FizzBuzz program. It prints every number from 1 to 100,
// except for multiples of 3 it prints "fizz" instead of the number, and for
// multiples of 5 it prints "buzz" instead of the number. If the number is
// both a multiple of 3 and 5, it prints "fizzbuzz".

// Fix the compile time error so that the program runs successfully.

fn main() {
    let mut n = 1;
    while n <= 100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}
