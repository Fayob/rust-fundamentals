struct Uppercase(String);

impl From<String> for Uppercase {
    fn from(value: String) -> Self {
        Uppercase(value.to_uppercase())
    }
}

impl From<&str> for Uppercase {
    fn from(data: &str) -> Self {
        Uppercase(data.to_uppercase())
    }
}

fn main() {
    let upper = Uppercase::from("lowercase");
    let upper_into: Uppercase = "lowercase".into();
    println!("{}, {}", upper.0, upper_into.0)
}
