use std::cmp::Ordering;

// PartialOrd can only compare the first field in struct by default
// if you want to compare another field in structs, you'll have to 
// implement the functionality in partialOrd manually

#[derive(PartialEq)] // PartialEq is neccessary for PartialOrd to work
struct User {
    id: i32,
    name: String,
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.name < other.name {
            Some(Ordering::Less)
        } else if self.name > other.name {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn main() {}
