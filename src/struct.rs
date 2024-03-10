struct Employee {
    name: String,
    age: i32,
    department: String,
    role: String,
}

fn print_employee_details(employee: Employee) {
    println!("name {:?}", employee.name);
    println!("age {:?}", employee.age);
    println!("department {:?}", employee.department);
    println!("role {:?}", employee.role)
}

enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sweet => print!("Sweet => "),
        Flavor::Fruity => print!("Fruity => "),
        Flavor::Sparkling => print!("Sparkling => "),
    }

    println!("oz: {:?}", drink.fluid_oz)
}

struct SchoolDB {
    name: String,
    age: i32,
    dept: String,
}

fn main() {
    let new_employee = Employee {
        name: "James".to_owned(),
        age: 22,
        department: "IT".to_owned(),
        role: "Software Engineer".to_owned(),
    };
    print_employee_details(new_employee);

    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 6.0,
    };
    print_drink(sweet);

    let fruity = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 12.0,
    };
    print_drink(fruity);

    let schooldb = SchoolDB {
        name: "James".to_owned(),
        age: 14,
        dept: String::from("Bio"),
    };

    match schooldb {
        SchoolDB { age: 12, name, .. } => println!("Student @ age 12 is  {:?}", name),
        SchoolDB { age, .. } => println!("student age => {:?}", age),
    }
}
