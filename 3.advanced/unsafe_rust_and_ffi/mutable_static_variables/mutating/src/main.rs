// Fix the code to make it compile.

static mut COUNTER: u32 = 0;

// return fibonacci number corresponding to specified position
fn fib(num: u32) -> u32 {
    unsafe {
        COUNTER += 1;
    }

    if num <= 1 {
        num
    } else {
        fib(num - 1) + fib(num - 2)
    }
}

fn main() {
    let num = fib(5);
    println!("Fibonacci number at position 5: {num}");

    unsafe {
        println!("Number of function calls made to calculate: {COUNTER}");
    }
}
