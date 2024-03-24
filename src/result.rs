#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice){
    println!("choice = {:?}", choice)
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

struct Customer {
    age: i32
}

fn try_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age < 21 {
        Err("customer must be at least 21".to_owned())
    } else {
        print!("Purchased... ");
        Ok(())
    }
}

enum Position {
    Maintenace,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech,
}

enum Status {
    Active,
    Terminated
}

struct Employee {
    position: Position,
    status: Status
}

fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("terminated".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::AssemblyTech => Ok(()),
        Position::LineSupervisor => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("invalid position".to_owned())
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    let attempt_access = try_access(employee)?;
    println!("access ok, {:?}", attempt_access);
    Ok(())
}


fn main() {
    let choice = get_choice("mainmenu");
    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error = {:?}", e)
    }
    let res = pick_choice("start");
    println!("{:?}", res);

    let ashley = Customer { age: 20};
    let purchased = try_purchase(&ashley);
    println!("{:?}", purchased);

    let manager = Employee {
        position: Position::Manager,
        status: Status::Active
    };
    let kichen_staff = Employee {
        position: Position::KitchenStaff,
        status: Status::Active
    };
    let line_supervisor = Employee {
        position: Position::LineSupervisor,
        status: Status::Terminated
    };
    match print_access(&manager) {
       Err(e)  => println!("{:?}", e),
       _ => (),
    }

    let ks_res = print_access(&kichen_staff);
    println!("{:?}", ks_res);

    let ls_res = print_access(&line_supervisor);
    println!("{:?}", ls_res);
}
