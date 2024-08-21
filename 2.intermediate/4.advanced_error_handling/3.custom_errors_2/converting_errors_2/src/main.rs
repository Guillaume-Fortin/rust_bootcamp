// Complete the `add` function by converting from ParseIntError to AddError using `map_err` combinator.

use std::num::ParseIntError;

#[derive(Debug)]
enum AddError {
    ParseError(ParseIntError),
    Overflow,
}

fn add(num1: &str, num2: &str) -> Result<u8, AddError> {
    let num1 = num1
        .parse::<u8>()
        .map_err(|e: ParseIntError| AddError::ParseError(e))?;

    let num2 = num2.parse::<u8>().map_err(|e| AddError::ParseError(e))?;

    num1.checked_add(num2).ok_or(AddError::Overflow)
}

fn main() {
    let (input1, input2) = ("23", "50");
    match add(input1, input2) {
        Ok(res) => println!("{input1} + {input2} = {res}"),
        Err(e) => println!("Error: {e:?}"),
    }
}
