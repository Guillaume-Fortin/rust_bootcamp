// Fix the code to make it compile. You may only add code, not remove it.

use std::time::Duration;
use tokio::time::sleep;

struct Employee {
    id: u32,
    name: String,
    salary: f32,
}

impl Employee {
    fn new(id: u32, name: &str, salary: f32) -> Self {
        Self {
            id,
            name: name.to_string(),
            salary,
        }
    }
}

#[tokio::main]
async fn main() {
    let (id1, id2) = (2, 3);
    let emp1 = read_details_from_db(id1).await.unwrap();
    let emp2 = read_details_from_db(id2).await.unwrap();
    if emp1.salary > emp2.salary {
        println!(
            "{} earns ${} more than {}",
            emp1.name,
            emp1.salary - emp2.salary,
            emp2.name
        );
    } else if emp2.salary > emp1.salary {
        println!(
            "{} earns ${} more than {}",
            emp2.name,
            emp2.salary - emp1.salary,
            emp1.name
        );
    } else {
        println!("Both {} and {} earn same amount", emp1.name, emp2.name);
    }
}

async fn read_details_from_db(id: u32) -> Result<Employee, String> {
    // dummy read from database
    sleep(Duration::from_millis(1000)).await;
    let database = [
        Employee::new(1, "Alice", 98000.0),
        Employee::new(2, "Bob", 95000.0),
        Employee::new(3, "Cindy", 95000.0),
    ];
    for emp in database {
        if id == emp.id {
            return Ok(emp);
        }
    }
    Err("Employee record not present".into())
}
