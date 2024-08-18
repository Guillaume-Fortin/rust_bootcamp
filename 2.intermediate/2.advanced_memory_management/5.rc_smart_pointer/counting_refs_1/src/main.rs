// We can get the number of references behind a Rc value using Rc::strong_count.
// Can you guess the number of references alive at various places?
// Only use numbers as second parameter to assert_eq's.

use std::rc::Rc;
fn main() {
    let ptr1 = Rc::new('🦀'); // A crab stored on the heap
    assert_eq!(Rc::strong_count(&ptr1), 1);
    {
        let ptr2 = Rc::clone(&ptr1);
        assert_eq!(Rc::strong_count(&ptr1), 2);
        {
            let ptr3 = Rc::clone(&ptr2);
            assert_eq!(Rc::strong_count(&ptr1), 3);
        }
        assert_eq!(Rc::strong_count(&ptr1), 2);
    }
    assert_eq!(Rc::strong_count(&ptr1), 1);
}
