enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main () {
    let name = "fay";
    match name {
        "fay" => println!("That's my nick name"),
        "bob" => println!("Hello Bob"),
        "Doe" => println!("Hello Doe"),
        "Jon" => println!("Hello Jon"),
        _ => println!("Hello {:?}", name) // this must be covered in match statement to
                                          // catch other values that's not in the option, 
                                          // if not the compiler will throw an error
    }

    let some_int = 2;
    match some_int {
        1 => println!("It's 1"),
        2 => println!("It's 2"),
        3 => println!("It's 3"),
        _ => println!("It's another number"),
    }

    let my_bool = true;
    match my_bool {
        true => println!("It's true"),
        false => println!("It's false"),
    }

    let tickets = vec![
        Ticket::Backstage(50.0, "Bob".to_owned()),
        Ticket::Standard(50.0),
        Ticket::Vip(60.0, String::from("Amy")),
       ];
    
       for ticket in tickets {
           match ticket {
               Ticket::Backstage(price, holder) => println!("Holder: {:?}, price: ${:?}", holder, price),
               Ticket::Standard(price) => println!("Standard Price => ${:?}", price),
               Ticket::Vip(price,name ) => println!("VIP name: {:?} & price = ${:?}", name, price)
           }
       }
}