// Make the code compile by completing the function signatures.

trait Shape {
    fn shape(&self) -> String;
}

struct Triangle;

struct Square;

impl Shape for Triangle {
    fn shape(&self) -> String {
        "🔺".to_string()
    }
}

impl Shape for Square {
    fn shape(&self) -> String {
        "🟥".to_string()
    }
}

fn get_shape(side_count: u8) -> Box<dyn Shape> {
    match side_count {
        3 => Box::new(Triangle),
        4 => Box::new(Square),
        _ => panic!("No shape with side count available"),
    }
}

fn draw_shape(to_draw: &dyn Shape) {
    println!("{}", to_draw.shape())
}

fn main() {
    let my_shape = get_shape(4);
    draw_shape(my_shape.as_ref());
}
