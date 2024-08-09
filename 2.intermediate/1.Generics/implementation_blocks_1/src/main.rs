// Implement the add method for Pair<i32> type.

use std::ops::Add;

struct Pair<T>(T, T);

// impl Pair<i32> {
//     fn add(&self) -> i32 {
//         self.0 + self.1
//     }
// }

impl<T> Pair<T>
where
    T: Add<Output = T> + Copy, // Add trait is implemented for T and Copy trait is implemented for T
{
    fn add(&self) -> T {
        self.0 + self.1
    }
}

fn main() {
    let p1 = Pair(10, 23);
    let addition = p1.add();
    assert_eq!(addition, 33);
}
