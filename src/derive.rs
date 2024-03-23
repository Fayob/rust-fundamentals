#[derive(Debug)]
enum Position {
    Manager,
    Supervisor,
    Worker
}

#[derive(Debug)]
struct Employee {
    position: Position,
    work_hour: i64,
}

fn main() {
    let employee = Employee {
        position: Position::Worker,
        work_hour: 20,
    };
    println!("{:?}", employee.position);
    println!("{:?}", employee);
    
}
