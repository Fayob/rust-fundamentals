struct Freinds {
    names: Vec<String>,
}

// For Moving variable
impl IntoIterator for Freinds {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.into_iter()
    }
}

// For borrowing variable
impl<'a> IntoIterator for &'a Freinds {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.iter()
    }
}

// For mutable variable
impl<'a> IntoIterator for &'a mut Freinds {
    type Item = &'a mut String;
    type IntoIter = std::slice::IterMut<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.iter_mut()
    }
}

impl Freinds {
    fn iter(&self) -> std::slice::Iter<'_, String> {
        self.into_iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, String> {
        self.into_iter()
    }
}

fn main() {
    let friends = Freinds {
        names: vec!["John".to_owned(), "Peter".to_owned()],
    };

    for f in &friends {
        println!("{:?}", f)
    }

    let total = friends.iter().count();
    println!("{:?}", total)
}
