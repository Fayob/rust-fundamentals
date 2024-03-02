// struct won't allow string slice (&str) type to be given to its field unless you explicitly specify a lifetime
// By default strings (like "Hello World") are string slice (&str) type in rust
// "&str" type can be converted to "String" using any these methods: String::from("James"), "James".to_owned()
// while "String" type can be converted into "&str" by using ampersand (&) in front of it's variable
struct Person {
    name: String,
    fav_color: String,
    age: i32
}

fn print(input: &str){
    println!("{:?}", input);
}

struct LineItem {
    name: String,
    count: i32,
} 

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn main() {
    let people = vec![
        Person {
            name: String::from("James"), // you c
            fav_color: "green".to_owned(),
            age: 12
        },
        Person {
            name: String::from("Peter"),
            fav_color: "yellow".to_owned(),
            age: 8
        },
        Person {
            name: String::from("John"),
            fav_color: "blue".to_owned(),
            age: 24
        },
        Person {
            name: String::from("George"),
            fav_color: "red".to_owned(),
            age: 6
        },
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color);
        }
    }

    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("fruit"),
            count: 3,
        },
    ];

    for item in receipt {
        print_name(&item.name);
        println!("count: {:?}", item.count)
    }
}
