use std::io;

fn get_input() -> io::Result<String> {
    println!("Please enter a word");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main() {
    let mut all_inputs = vec![];
    let mut times_input = 0;

    while times_input < 2 {
        match get_input() {
            Ok(words) => {
                all_inputs.push(words);
                times_input += 1;
            }
            Err(e) => println!("error: {:?}", e),
        }
    }

    for input in all_inputs {
        println!(
            "Original => {:?}\nCapitalized => {:?}",
            input,
            input.to_ascii_uppercase()
        )
    }
}
