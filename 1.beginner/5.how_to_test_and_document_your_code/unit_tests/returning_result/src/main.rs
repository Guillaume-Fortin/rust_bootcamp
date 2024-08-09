// Complete the test function's signature.

fn calculate_sum(nums: &[i32]) -> Result<i32, String> {
    if nums.len() == 0 {
        return Err("Number list is empty".to_string());
    }
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_sum_correctly() -> Result<(), String> {
        // return String Error because we use Result<i32, String> in calculate_sum and use question mark operator (?)
        let nums = [1, 2, 3, 4, 5];
        let sum = calculate_sum(&nums)?;
        assert_eq!(sum, 5 * (5 + 1) / 2);
        Ok(())
    }
}
