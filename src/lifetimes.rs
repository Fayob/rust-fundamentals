fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if second.len() > first.len() {
        second
    } else {
        first
    }
}

struct Cards {
    inner: Vec<IdCard>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum City {
    Barland,
    Bazopolis,
    Fooville,
}

#[derive(Debug)]
struct IdCard {
    name: String,
    age: u8,
    city: City,
}

impl IdCard {
    pub fn new(name: &str, age: u8, city: City) -> Self {
        Self {
            name: name.to_owned(),
            age,
            city,
        }
    }
}

fn new_ids() -> Cards {
    Cards {
        inner: vec![
            IdCard::new("Amy", 1, City::Fooville),
            IdCard::new("Matt", 10, City::Barland),
            IdCard::new("Bailee", 20, City::Barland),
            IdCard::new("Anthony", 30, City::Bazopolis),
            IdCard::new("Tina", 40, City::Bazopolis),
        ],
    }
}

struct YoungPeople<'a> {
    inner: Vec<&'a IdCard>,
}

impl<'a> YoungPeople<'a> {
    fn living_in_fooville(&self) -> Self {
        Self {
            inner: self
                .inner
                .iter()
                .filter(|id| id.city == City::Fooville)
                .map(|id| *id)
                .collect(),
        }
    }
}

fn main() {
    let short = "hello";
    let long = "this is the long message";
    println!("{:?}", longest(short, long));

    let ids = new_ids();

    let young = YoungPeople {
        inner: ids.inner.iter().filter(|id| id.age <= 20).collect(),
    };

    println!("ids");
    for id in ids.inner.iter() {
        println!("{:?}", id)
    }

    println!("young");
    for id in young.inner.iter() {
        println!("{:?}", id)
    }

    println!("living in fooville");
    for id in young.living_in_fooville().inner {
        println!("{:?}", id)
    }
}
