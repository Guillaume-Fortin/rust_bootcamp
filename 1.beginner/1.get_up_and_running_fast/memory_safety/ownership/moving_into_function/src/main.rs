// Fix the code so that it compiles.

fn main() {
    let my_string = String::from("I love rust bootcamp 💕");
    let occurence_count = count_occurences(&my_string, 'o');
    println!("The number of times 'o' apprears in \"{my_string}\" = {occurence_count}");
}

// this function counts the number of times a letter appears in a text
fn count_occurences(text: &String, letter: char) -> u32 {
    let mut res = 0;
    for ch in text.chars() {
        if ch == letter {
            res += 1;
        }
    }
    res
}
