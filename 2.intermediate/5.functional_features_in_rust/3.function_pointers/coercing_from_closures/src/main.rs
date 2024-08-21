// Fix the code by addressing the TODO.

fn invoker<T>(logic: fn(T), arg: T) {
    logic(arg);
}

fn main() {
    // TODO: shift below declaration to somewhere else
    //let greeting = String::from("Nice to meet you");
    let greet = |name| {
        let greeting = String::from("Nice to meet you");
        println!("{greeting} {name}!");
    };
    invoker(greet, "Jenny");
}
