// Fix the code to make it compile. You may not modify any statement.

fn main() {
    let num = 123;

    // &num is a reference to the value of num
    // *const i32 is a raw pointer to a i32 value
    let ptr = &num as *const i32;

    unsafe {
        println!("{} stored at {:p}", *ptr, ptr);
    }
}
