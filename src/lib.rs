pub mod fizzy {
    pub fn fizzbuzz(v: i32) -> String {
        let mut result = String::new();

        if v <= 0 {
            let s: String = v.to_string();
            result.push_str(&s.to_owned());
        } else if v % (3*5) == 0 {
            result.push_str("fizzbuzz");
        } else if v % 3 == 0 {
            result.push_str("fizz");
        } else if v % 5 == 0{
            result.push_str("buzz");
        } else {
            let s: String = v.to_string();
            result.push_str(&s.to_owned());
        }

        return result;
    }
}


#[cfg(test)]
mod tests {
    use super::fizzy::fizzbuzz;

    #[test]
    fn it_calls() {
        assert_eq!("0", fizzbuzz(0));
    }

    #[test]
    fn three_yields_fizz() {
        assert_eq!("fizz", fizzbuzz(3));
    }
    #[test]
    fn five_yields_buzz() {
        assert_eq!("buzz", fizzbuzz(5));
    }
    #[test]
    fn fifteen_yields_fizzbuzz() {
        assert_eq!("fizzbuzz", fizzbuzz(15));
    }
    #[test]
    fn thirty_yields_fizzbuzz() {
        assert_eq!("fizzbuzz", fizzbuzz(30));
    }
}
