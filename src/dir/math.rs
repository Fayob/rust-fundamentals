pub fn divide(a: i32, b: i32) {
    if b == 0 {
        println!("Error!");
    } else {
        let res = a / b;
        print!("{:?}/{:?} = {:?}", a, b, res);
    }
}
