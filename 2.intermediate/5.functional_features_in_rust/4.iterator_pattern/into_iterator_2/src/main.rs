// Fix the code by completing the into_iter() method.

struct Employee {
    first_name: String,
    last_name: String,
    id: String,
}

impl IntoIterator for Employee {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;
    fn into_iter(self) -> Self::IntoIter {
        vec![
            format!("First name: {}", self.first_name),
            // do the same for last_name & id
            format!("Last name: {}", self.last_name),
            format!("Id: {}", self.id),
        ]
        .into_iter()
    }
}

fn main() {
    let employee = Employee {
        first_name: "Alice".to_owned(),
        last_name: "Smith".to_owned(),
        id: "ab123".to_owned(),
    };
    println!("Employee Details:");
    for detail in employee {
        println!("{detail}");
    }
}
