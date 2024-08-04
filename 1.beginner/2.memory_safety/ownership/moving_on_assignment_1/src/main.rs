// Something's missing. Fix the code so that it compiles.

fn main() {
    let s1 = String::from("Rust");
    let mut s2 = s1.clone();
    s2.push_str(" is an awesome language");
    println!("String:\"{s1}\" is a substring of \"{s2}\"");
}
