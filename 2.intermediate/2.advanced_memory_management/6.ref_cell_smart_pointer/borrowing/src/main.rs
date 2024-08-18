// Complete the following code.

use std::cell::RefCell;

fn main() {
    // storing value 5 on heap
    let ptr = RefCell::new(5);
    // get an immutable reference to the stored value
    let ref1 = ptr.borrow();
    println!("Stored value: {}", ref1);
    drop(ref1);
    // get a mutable reference to the stored value
    let mut ref2 = ptr.borrow_mut();
    *ref2 = 6; // Note: we can mutate the value associated with ptr, even though it is not marked as mut
    println!("Stored value: {}", ref2);
}
