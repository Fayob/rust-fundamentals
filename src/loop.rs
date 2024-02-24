fn main() {
    let mut i = 3;
    // infiite loop
    loop {
        println!("{:?}", i);
        i = i - 1;
        if i == 0 {
            break;
        }
    }
    println!("Done!");

    let mut n = 1;
    loop {
        println!("{:?}", n);
        if n == 4 {
            break;
        }
        n += 1;
    }

    // while loop
    let mut i = 1;
    while i <= 3 {
        println!("{:?}", i);
        i += 1;
    }

    let mut counter = 5;
    while counter >= 1 {
        println!("Here is the number => {:?}", counter);
        counter -= 1;
    }
}
