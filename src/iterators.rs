fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let plus_one: Vec<_> = numbers.iter().map(|num| num + 1).collect();
    println!("{:?}", plus_one);

    let new_numbers: Vec<_> = numbers.iter().filter(|num| *num % 2 == 0).collect();
    println!("{:?}", new_numbers);

    let find_me = numbers.iter().find(|num| *num == &4);
    println!("{:?}", find_me);
    
    let count = numbers.iter().count();
    println!("{:?}", count);

    let last = numbers.iter().last();
    println!("{:?}", last);

    let min = numbers.iter().min();
    println!("{:?}", min);

    let max = numbers.iter().max();
    println!("{:?}", max);

    let take: Vec<_> = numbers.iter().take(4).collect();
    println!("{:?}", take);

    let data = vec![1, 2, 3, 4, 5, 6];

    let new_data: Vec<i32> = data
        .iter()
        .map(|num| num * 3)
        .filter(|num| num > &10)
        .collect();

    for num in new_data {
        print!("{:?},", num)
    }

}
