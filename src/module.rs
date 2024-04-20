// Inline Module

mod greet {
    pub fn greeting() {
        println!("Good morning...")
    }
}

mod msg {
    pub fn capitalize(msg: &str) {
        println!("{:?}", msg.to_uppercase())
    }

    pub fn repeat_word(msg: &str) {
        println!("{:?}", msg.repeat(2))
    }

    pub fn word_length(msg: &str) {
        println!("The length of the message is =>{:?}", msg.len())
    }
}

mod math {
    pub fn add(a: i32, b: i32) {
        let sum = a + b;
        println!("The sum of {:?}, and {:?} is: {:?}", a, b, sum);
    }

    pub fn mul(a: i32, b: i32) {
        let res = a * b;
        println!("The multiplication of {:?}, and {:?} is: {:?}", a, b, res);
    }

    pub fn sub(a: i32, b: i32) {
        let ans = a - b;
        println!("The subtraction of {:?} from {:?} is: {:?}", b, a, ans);
    }
}

fn main() {
    use math::*; // module can be import like this

    {
        // here is another way of importing a module
        use msg::{capitalize, repeat_word, word_length};

        capitalize("capitalize");
        repeat_word("repeat_word");
        word_length("word_length");
    }

    math::add(1, 2); // functions/parameters in a module can be access like this
    mul(2, 4); // functions/parameters in a module can as well be used like this
    sub(4, 2);
    greet::greeting(); // functions/parameters in a module can be access this way even when it's not imported into the file
}
