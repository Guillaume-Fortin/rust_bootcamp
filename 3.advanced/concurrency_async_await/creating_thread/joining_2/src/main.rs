// Fix the code to make sure that sum is calculated correctly.

use std::{
    ops::Range,
    thread::{self, JoinHandle},
};

fn summation_thread(range: Range<u64>) -> JoinHandle<u64> {
    thread::spawn(|| {
        let mut sum = 0;
        for num in range {
            sum += num;
        }
        sum
    })
}

// calculate the sum of numbers 1..=40_00_000 using four threads
fn main() {
    let handle1 = summation_thread(1..10_00_000);
    let handle2 = summation_thread(10_00_000..20_00_000);
    let handle3 = summation_thread(20_00_000..30_00_000);

    let mut sum = 0u64;
    sum += handle1.join().unwrap();
    sum += handle2.join().unwrap();
    sum += handle3.join().unwrap();

    for num in 30_00_000..=40_00_000 {
        sum += num;
    }
    println!("Sum of numbers 1..=40_00_000: {sum}");
    assert_eq!(sum, 8000002000000);
}
