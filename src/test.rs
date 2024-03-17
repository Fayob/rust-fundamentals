use std::fmt::format;

fn all_caps(word: &str) -> String {
    word.to_uppercase().to_owned()
}

fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {
   
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_all_caps(){
        let res = all_caps("hello");
        let expected_res = String::from("HELLO");
        assert_eq!(res, expected_res, "string should all be in uppercase");
    }

    #[test]
    fn test_clamp() {
        let res1 = clamp(6, 4, 8);
        let res1_expected_res = 6;
        let res2 = clamp(2, 12, 3);
        let res2_expected_res = 12;
        let res3 = clamp(9,1, 7);
        let res3_expected_res = 7;
        let res4 = clamp(5, 5, 5);
        let res4_expected_res = 5;

        assert_eq!(res1, res1_expected_res, "should n only when n is not lesser than lower and greater than upper");
        assert_eq!(res2, res2_expected_res, "should n only when n is not lesser than lower and greater than upper");
        assert_eq!(res3, res3_expected_res, "should n only when n is not lesser than lower and greater than upper");
        assert_eq!(res4, res4_expected_res, "should n only when n is not lesser than lower and greater than upper");
    }

    #[test]
    fn test_divide() {
        let div = divide(4, 2);
        let div2 = divide(1, 0);

        assert_eq!(div, Some(2), "It should return option of interger");
        assert_eq!(div2, None, "It should return none")
    }

    #[test]
    fn check_concat() {
        let ans = concat("hello", "world");

        assert_eq!(ans, "helloworld".to_owned(), "Should retrun String typr")
    }
}
