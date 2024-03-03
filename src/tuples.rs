fn print_student_info(name: &str, age: i32, class: &str) -> (String, i32, String) {
    return (name.to_owned(), age, class.to_owned());
}

fn coordinate() -> (i32, i32) {
    (2, 8)
}

fn main() {
    let coord = (2, 3, 4, 6);
    println!("{:?}, {:?}, {:?}, {:?}", coord.0, coord.1, coord.2, coord.3);

    let (a, b, c, d) = coord;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let (name, age) = ("Ade", 14);
    println!("The boy's name is {:?} and his age is {:?}", name, age);

    let (student1_name, student1_age, student1_class) = print_student_info("John", 12, "Rust");
    let (student2_name, student2_age, student2_class) = print_student_info("Peter", 14, "Java");
    println!(
        "Student Name: {:?}, Student Age: {:?}, Student Class {:?}",
        student1_name, student1_age, student1_class
    );
    println!(
        "Student Name: {:?}, Student Age: {:?}, Student Class {:?}",
        student2_name, student2_age, student2_class
    );

    let (x, y) = coordinate();
    if x > 2 {
        println!("{:?}", y)
    } else {
        println!("coordinate value x is not greater than 2")
    }
}
