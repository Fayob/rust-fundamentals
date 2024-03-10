enum DataBase {
   SCHOOLDATABASE, 
   HOSPITALDATABASE, 
   COMPANYDATABASE,
   GOVERNMENTDATABASE
}

fn enumfn(enum_variant: DataBase) -> String {
    match enum_variant {
        DataBase::COMPANYDATABASE => "Company database".to_owned(),
        DataBase::SCHOOLDATABASE => "School database".to_owned(),
        DataBase::HOSPITALDATABASE => "Hospital database".to_owned(),
        DataBase::GOVERNMENTDATABASE => "Government database".to_owned(),
    }
}

enum Color {
    Blue, 
    Yellow, 
    Green,
    Red
 }
 
 fn print_color(my_color: Color) {
 
     match my_color {
         Color::Blue => println!("My Color is => Blue"),
         Color::Yellow => println!("Yellow"),
         Color::Green => println!("Green"),
         Color::Red => println!("Red"),
     }
 }

 // More on Enum
 enum Discount {
    Percentage(i32),
    Flat(i32),
}


fn main() {
    let dbcom = DataBase::COMPANYDATABASE;
    let dbsch = DataBase::SCHOOLDATABASE;
    let result_com = enumfn(dbcom);
    let result_sch = enumfn(dbsch);
    let result_hos = enumfn(DataBase::HOSPITALDATABASE);
    let result_gov = enumfn(DataBase::GOVERNMENTDATABASE);
    println!("{:?}", result_com);
    println!("{:?}", result_sch);
    println!("{:?}", result_hos);
    println!("{:?}", result_gov);

    print_color(Color::Blue);
    print_color(Color::Yellow);

    // more on Enum example
    let discount = Discount::Percentage(25);

    match discount {
        Discount::Percentage(20) => println!("20% of discount given"),
        Discount::Percentage(val) => println!("Percentage => {:?}", val),
        Discount::Flat(amt) => println!("Flat Value => {:?}", amt),
        _ => println!("Some Other Discount value")
    }
}