enum Color {
    Yellow,
    Blue,
    Green
}

fn main() {
    // if let can be used when you want to match just one option 
    // without bothering about the remaining options
    let maybe_user = Some("Jerry");
    if let Some(user) = maybe_user {
        println!("user = {:?}", user);
    }

    let yellow = Color::Yellow;
    if let Color::Yellow = yellow {
        println!("It's yellow")
    }
}