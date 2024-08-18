// Fix the code by shifting only one statement.

fn main() {
    let str1 = "🦀".to_string();
    println!("A crab: {str1}");

    let bytes = str1.into_bytes();
    println!("A crab represented in unicode: {bytes:?}");
}
