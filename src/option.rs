fn returning_option_int() -> Option<i32> {
    Some(32)
}
fn returning_option_str() -> Option<String> {
    Some("Hello word".to_owned())
}
fn returning_option_none() -> Option<String> {
    None
}

struct GroceryItem {
    name: String,
    quantity: Option<i32>,
}

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    println!("{:?}", returning_option_int());
    println!("{:?}", returning_option_none());
    println!("{:?}", returning_option_str());

    let grocery_items = vec![
        GroceryItem {
            name: "butter".to_owned(),
            quantity: Some(4),
        },
        GroceryItem {
            name: "egg".to_owned(),
            quantity: Some(12),
        },
        GroceryItem {
            name: "burger".to_owned(),
            quantity: None,
        },
        GroceryItem {
            name: "pizza".to_owned(),
            quantity: Some(2),
        },
    ];

    for grocery_item in grocery_items {
        // if groceryItem.quantity != None {
        //     println!("{:?} has {:?} quantity(s)", groceryItem.name, groceryItem.quantity);
        // }
        match grocery_item.quantity {
            Some(qty) => println!("{:?} has {:?} quantity(s)", grocery_item.name, qty),
            None => println!("{:?} has no quantity", grocery_item.name),
        }
    }

    let response = Survey{
        q1: Some(12),
        q2: Some(true),
        q3: None,
    };

    match response.q1 {
        Some(ans) => println!("q1: {:?}", ans),
        None => println!("No response")
    }

    match response.q2 {
        Some(ans) => println!("q2: {:?}", ans),
        None => println!("No response")
    }

    match response.q3 {
        Some(ans) => println!("q3: {:?}", ans),
        None => println!("No response")
    }
}
