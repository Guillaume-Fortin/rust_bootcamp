// Fix the code so that it compiles.

fn main() {
    let side_count = 5;
    let message = match side_count {
        0 | 1 | 2 => "invalid shape",
        3 => "it's a triangle",
        4 => "it's a quadrilateral",
        5 => "it's a pentagon",
        6 => "it's a hexagon",
        _ => "i don't know the name, lol",
    };
    println!("{message}");
}
