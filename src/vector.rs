struct Test {
    score:i32,
}
fn main() {
    let mut new_vec = Vec::new();
    new_vec.push(1);
    new_vec.push(2);
    new_vec.push(3);
    new_vec.push(4);
    new_vec.push(5);
    new_vec.pop();
    println!("vec length {:?}\nvec isempty? {:?}\n", new_vec.len(), new_vec.is_empty());

   let my_score = vec![
    Test { score: 100 },
    Test { score: 80 },
    Test { score: 90 },
    Test { score: 95 },
    Test { score: 98 },
   ];

   for test in my_score {
    println!("score: {:?}", test.score)
   }

   let my_num = vec![10, 20, 30, 40];

    for num in &my_num {
        if num == &30 {
            println!("Thirty")
        } else {
            println!("{:?}", num)
        }
    }

    println!("number of elements = {:?}", my_num.len());
}
