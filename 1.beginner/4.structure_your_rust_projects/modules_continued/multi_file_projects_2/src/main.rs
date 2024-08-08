// Complete the code by use declarations above main.

mod student {
    pub mod operations {
        use super::Student; // using super to refer to parent module

        pub fn assign_grade(student: &mut Student) {
            if student.marks >= 80 {
                student.grade = 'A';
            } else if student.marks >= 60 {
                student.grade = 'B';
            } else {
                student.grade = 'C';
            }
        }
    }

    pub struct Student {
        pub name: String, // struct fields can also be made public
        pub marks: u8,
        pub grade: char,
    }

    impl Student {
        // make methods/associated functions public in order to access from outside the module
        pub fn new(name: &str, marks: u8) -> Self {
            Self {
                name: name.to_string(),
                marks,
                grade: 'X',
            }
        }
    }
}

// use student::operations::assign_grade;
// use student::Student;
use student::{operations::assign_grade, Student};

fn main() {
    let mut student = Student::new("Alice", 75);
    assign_grade(&mut student);
    println!("{} got {} grade", student.name, student.grade);
}
