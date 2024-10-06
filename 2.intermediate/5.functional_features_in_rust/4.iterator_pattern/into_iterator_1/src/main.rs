// Provide the trait implementations and make the code execute successfully.

struct Employee {
    first_name: String,
    last_name: String,
    id: String,
}

struct EmployeeIter {
    state: Vec<String>,
}

impl Iterator for EmployeeIter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.state.is_empty() {
            None
        } else {
            Some(self.state.remove(0))
        }
    }
}

impl IntoIterator for Employee {
    type Item = String;
    type IntoIter = EmployeeIter;
    fn into_iter(self) -> Self::IntoIter {
        EmployeeIter {
            state: vec![self.first_name, self.last_name, self.id],
        }
    }
}

fn main() {
    let employee = Employee {
        first_name: "Alice".to_owned(),
        last_name: "Smith".to_owned(),
        id: "ab123".to_owned(),
    };
    let mut emp_iter = employee.into_iter();
    println!("First name: {}", emp_iter.next().unwrap());
    println!("Last name: {}", emp_iter.next().unwrap());
    println!("ID: {}", emp_iter.next().unwrap());
    assert_eq!(emp_iter.next(), None);
}
