use num::integer::sqrt;

// There's a prime number hiding in our array of integers!
// The function below tries to find the prime number by checking each element,
// and finding its divisor. If none is found, then it's a prime number and
// its search ends!

// But it seems that its search never does end, when there's clearly a
// prime number there. Fix the function so that it returns the prime number.

fn main() {
    let numbers = [36, 25, 49, 3, 64, 16, 9];
    let prime = get_prime(numbers);
    println!("result: {prime}")
}

fn get_prime(arr: [i32; 7]) -> i32 {
    // Loop through every element in the array
    let mut i = 0;
    'outer: loop {
        // Find a divisor.
        let mut n = 2;
        'inner: loop {
            // If a number can be evenly divided by another number except 1 and itself,
            // then it's not a prime.
            // Break out here to move on to the next element.
            if arr[i] % n == 0 {
                if arr[i] == 2 {
                    break 'outer;
                }
                i += 1;
                break;
            }

            // If no divisors are found, then we've found a prime!
            // Break out of the loop here.
            if n >= sqrt(arr[i]) {
                break 'outer;
            }

            // Otherwise, move to the next element.
            n += 1;
        }
    }
    println!("The first prime number in the array is {}.", arr[i]);
    arr[i]
}
