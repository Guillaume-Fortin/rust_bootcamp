// Fix the code to make it compile.

fn main() {
    // print first 10 fibonacci numbers
    let (mut a, mut b) = (1, 0);
    let mut c = 0;
    let ptr_a = &mut a as *mut i32;
    let ptr_b = &mut b as *mut i32;
    let ptr_c = &mut c as *mut i32;

    unsafe {
        for _ in 0..10 {
            *ptr_c = *ptr_a + *ptr_b;
            println!("{}", *ptr_c);
            *ptr_a = *ptr_b;
            *ptr_b = *ptr_c;
        }
    }
}
