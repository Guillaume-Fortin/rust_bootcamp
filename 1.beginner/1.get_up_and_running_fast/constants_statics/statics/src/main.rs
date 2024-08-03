// Fix the code so it compiles!

static LANGUAGE: &str = "Golang";

fn main() {
    // These two initializations perform a string copy from same memory location
    let lang1 = LANGUAGE;
    let mut lang2 = LANGUAGE;

    lang2 = "Rust";
    println!("I like {} more than {}!", lang2, lang1);
}
