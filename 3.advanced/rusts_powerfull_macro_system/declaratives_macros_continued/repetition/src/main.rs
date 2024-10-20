// Complete the definition of `sum`.

macro_rules! sum {
    ($( $a:expr ),+) => {
        {
            let mut sum = 0;
            $( sum += $a; )+
            sum
        }
    };
}

fn main() {
    assert_eq!(sum!(1, 2, 3), 6);
    assert_eq!(sum!(10u8, 20u8), 30);
}
