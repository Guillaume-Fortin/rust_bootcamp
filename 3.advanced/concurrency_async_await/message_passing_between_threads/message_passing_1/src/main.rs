// Fix the code to make it compile and produce the correct output.

use std::sync::mpsc::{self, Sender};
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let nums = [12, 43, 54, 43, 53, 52, 98, 89];
    sum(&nums, 3, tx);

    //drop(tx); // drop tx so that rx will stop waiting for new values

    let mut res = 0;
    for sum in rx {
        res += sum;
    }
    println!("Sum of numbers: {res}");
}

// calculate sum of numbers using specified number of threads
fn sum(nums: &[i32], thread_count: usize, tx: Sender<i32>) {
    let elements_per_thread = nums.len() / thread_count;
    let mut start_pos;
    let mut partition;
    for i in 0..thread_count - 1 {
        start_pos = i * elements_per_thread;
        partition = Vec::from(&nums[start_pos..start_pos + elements_per_thread]);
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let mut sum = 0;
            for num in partition {
                sum += num;
            }

            tx_clone.send(sum).unwrap();
        });
    }
    // sum the remaining elements using last thread
    partition = Vec::from(&nums[(thread_count - 1) * elements_per_thread..]);
    thread::spawn(move || {
        let mut sum = 0;
        for num in partition {
            sum += num;
        }
        tx.send(sum).unwrap();
    });
}
