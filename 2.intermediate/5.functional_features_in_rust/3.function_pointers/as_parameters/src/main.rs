// Complete the function signature for `factorial`. It must not contain any generics/traits.

fn decrement(x: u32) -> u32 {
    x - 1
}

fn multiply(x: u32, y: u32) -> u32 {
    x * y
}

fn factorial(num: u32, dec: fn(u32) -> u32, mul: fn(u32, u32) -> u32) -> u32 {
    let mut res = 1;
    let mut temp = num;
    while temp > 1 {
        res = mul(res, temp);
        temp = dec(temp);
    }
    res
}

fn main() {
    let num = 6;
    let fact = factorial(num, decrement, multiply);
    println!("{num}! = {fact}");
}
