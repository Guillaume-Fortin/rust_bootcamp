// Make the code compile by only modifying the loop.

fn main() {
    let mut nums = [0, 1, 2, 3, 4];
    let odd_nums = [1, 3, 5, 7, 9];
    for num in nums.iter_mut() {
        *num = 2 * *num + 1;
    }
    assert_eq!(nums, odd_nums)
}
