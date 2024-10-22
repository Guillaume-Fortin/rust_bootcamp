// Fix the code to make it compile. You can only add and not remove anything from the code.

unsafe fn increment(a: *mut i32) {
    *a += 1;
}

unsafe fn get_val(a: *const i32) -> i32 {
    *a
}

fn main() {
    let mut x = 0;
    let ptr1 = &mut x as *mut i32;
    unsafe {
        increment(ptr1);
        increment(ptr1);
        increment(ptr1);
        assert_eq!(get_val(ptr1), 3);
    }
}
