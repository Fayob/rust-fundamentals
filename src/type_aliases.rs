use std::collections::HashMap;

struct Contact {
    name: String,
    phone: String,
}

type ContactName = String;
type ContactIndex = HashMap<ContactName, Contact>;

fn add_contact(index: &mut ContactIndex, contact: Contact){
    index.insert(contact.phone.to_owned(), contact);
}

// more examples

type Miles = u64;
type Centimeters = u64;

type BorrowedItems<'a> = Vec<&'a str>;

fn main() {}
