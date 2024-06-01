use std::thread;
use std::time::Duration;

fn main() {
    let iterations = 10;
    let a = thread::spawn(move || {
        for i in 1..=iterations {
            println!("A:{}", i)
        }
    });

    let b = thread::spawn(move || {
        for i in 1..=iterations {
            println!("        B:{}", i)
        }
    });

    let _ = a.join();
    let _ = b.join();

    // Another example
    let value = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });

    println!("Waiting on thread");

    match value.join() {
        Ok(temp) => println!("value: {}", temp),
        Err(e) => println!("error: {:?}", e),
    }

    // Another Example
    let data = vec!['a', 'b', 'c'];
    let caps = thread::spawn(move || {
        let data: Vec<_> = data.iter().map(|d| d.to_ascii_uppercase()).collect();
        data
    });

    println!("Waiting on value...");

    match caps.join() {
        Ok(temp) => println!("value: {:?}", temp),
        Err(e) => println!("error: {:?}", e),
    }
}