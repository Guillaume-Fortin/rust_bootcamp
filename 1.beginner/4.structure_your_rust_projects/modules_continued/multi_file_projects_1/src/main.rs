// Complete the code by bringing the required items into scope.
use days::{is_holiday, WeekDay};

mod days {
    pub enum WeekDay {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }

    pub fn is_holiday(day: &WeekDay) -> bool {
        match day {
            WeekDay::Sunday | WeekDay::Saturday => true,
            _ => false,
        }
    }
}

fn main() {
    let today = WeekDay::Friday;
    if is_holiday(&today) {
        println!("I can go out!");
    } else {
        println!("I have to work today!");
    }
}
