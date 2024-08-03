// Add a type alias for our dogs so we can store their names and ages.

fn main() {
    type Dog = (String, u16); /* Finish this line */

    let dog1: Dog = (String::from("Albert"), 3);
    println!("My dog {} is {} years old.", dog1.0, dog1.1);

    let dog2: Dog = (String::from("Barker"), 5);
    println!("My other dog {} is {} years old.", dog2.0, dog2.1);
}
