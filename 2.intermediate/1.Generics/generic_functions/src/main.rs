// Fix the code so that it compiles.

fn take_and_give_ownership<T>(input: T) -> T {
    input
}

struct User {
    name: String,
    id: u32,
}

fn main() {
    let str1 = String::from("Ferris the 🦀!");
    let user1 = User {
        name: "Alice".to_string(),
        id: 199,
    };
    let _str2 = take_and_give_ownership(str1);
    let _user2 = take_and_give_ownership(user1);
}
