// Fix the code to make it compile.

macro_rules! ptr {
    ($type:ty, $var:ident) => {
        &mut $var as *mut $type
    };
}

fn main() {
    let mut x = 20;
    let ptr1 = ptr!(i32, x);
    let ptr2 = ptr!(i32, x);
    println!("x: {x}");

    unsafe {
        *ptr1 = *ptr1 * 2;
        *ptr2 = *ptr2 * 2;
        *ptr2 = *ptr1 * 2;
        println!("x * 8 = {x}");
    }
}
