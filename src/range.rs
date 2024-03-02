fn main() {
    let a_to_d_range = "a".."e";
    let one_to_four_range = 1..=4;
    println!("{:?} ==> {:?}", a_to_d_range, one_to_four_range);

    for num in 1..=4 {
        println!("{:?}", num)
    }

    for char in 'a'..='e' {
        println!("{:?}", char)
    }
}
