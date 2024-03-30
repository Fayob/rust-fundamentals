#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}
fn main() {
    let a = Some(1);
    dbg!(a);
    
    let a_is_some = a.is_some();
    dbg!(a_is_some);

    let a_is_none = a.is_none();
    dbg!(a_is_none);

    let a_mapped = a.map(|num| num + 1); 
    dbg!(a_mapped);

    let a_filtered = a.filter(|num| num == &1);
    dbg!(a_filtered);

    let a_or_else = a.or_else(|| Some(5));
    dbg!(a_or_else);

    let a_unwrapped = a.unwrap_or_else(|| 2);
    dbg!(a_unwrapped);
}
