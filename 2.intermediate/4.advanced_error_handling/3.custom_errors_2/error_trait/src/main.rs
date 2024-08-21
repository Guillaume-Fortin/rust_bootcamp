// Complete the code and make the tests pass by implementing std::error::Error for CalculationError
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CalculationError {
    InvalidOperation(char),
    InvalidOperand(String),
    DivideByZero { dividend: i8 },
    Overflow,
}

impl Display for CalculationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::InvalidOperation(op) => {
                format!("{op} is not a valid operation. Allowed: +,-,/,*")
            }
            Self::InvalidOperand(num) => {
                format!("{num} is not a valid integer in range [-128, 127]")
            }
            Self::DivideByZero { dividend } => {
                format!("Can not divide by zero. Attempting to divide {dividend} by 0")
            }
            Self::Overflow => format!("Overflow while performing the operation"),
        };
        f.write_str(&msg)
    }
}

impl Error for CalculationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub fn calculate(num1: &str, num2: &str, operation: char) -> Result<i8, CalculationError> {
    let num1 = num1
        .parse::<i8>()
        .map_err(|_| CalculationError::InvalidOperand(num1.to_owned()))?;
    let num2 = num2
        .parse::<i8>()
        .map_err(|_| CalculationError::InvalidOperand(num2.to_owned()))?;
    match operation {
        '+' => num1.checked_add(num2).ok_or(CalculationError::Overflow),
        '-' => num1.checked_sub(num2).ok_or(CalculationError::Overflow),
        '*' => num1.checked_mul(num2).ok_or(CalculationError::Overflow),
        '/' => {
            if num2 == 0 {
                return Err(CalculationError::DivideByZero { dividend: num1 });
            }
            num1.checked_div(num2).ok_or(CalculationError::Overflow)
        }
        _ => Err(CalculationError::InvalidOperation(operation)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Error {
        e: Box<dyn std::error::Error>,
    }
    impl Error {
        // presence of this method ensures that std::error::Error is satisfied for CalculationError
        fn new(err: CalculationError) -> Self {
            Self { e: Box::new(err) }
        }
    }

    #[test]
    fn invalid_operation() {
        let res1 = calculate("12", "20", '$');
        let res2 = calculate("45", "43", '^');
        match (res1, res2) {
            (Err(e1), Err(e2)) => {
                assert_eq!(
                    "$ is not a valid operation. Allowed: +,-,/,*",
                    format!("{}", e1)
                );
                assert_eq!(
                    "^ is not a valid operation. Allowed: +,-,/,*",
                    format!("{}", e2)
                );
            }
            _ => panic!("Error expected!"),
        }
    }

    #[test]
    fn invalid_operand() {
        let res1 = calculate("ab", "3r", '+');
        let res2 = calculate("45", "4.23", '^');
        match (res1, res2) {
            (Err(e1), Err(e2)) => {
                assert_eq!(
                    "ab is not a valid integer in range [-128, 127]",
                    format!("{}", e1)
                );
                assert_eq!(
                    "4.23 is not a valid integer in range [-128, 127]",
                    format!("{}", e2)
                );
            }
            _ => panic!("Error expected!"),
        }
    }

    #[test]
    fn divide_by_zero() {
        let res1 = calculate("45", "0", '/');
        let res2 = calculate("70", "0", '/');
        match (res1, res2) {
            (Err(e1), Err(e2)) => {
                assert_eq!(
                    "Can not divide by zero. Attempting to divide 45 by 0",
                    format!("{}", e1)
                );
                assert_eq!(
                    "Can not divide by zero. Attempting to divide 70 by 0",
                    format!("{}", e2)
                );
            }
            _ => panic!("Error expected!"),
        }
    }

    #[test]
    fn overflow() {
        let res = calculate("120", "120", '+');
        match res {
            Err(e) => {
                assert_eq!("Overflow while performing the operation", format!("{}", e));
            }
            _ => panic!("Error expected!"),
        }
    }
}
