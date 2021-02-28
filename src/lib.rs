pub mod fizzy {
    pub fn fizzbuzz(v: i32) -> String {

        if v <= 0 {
            return v.to_string();
        } else if v % (3*5) == 0 {
            return "fizzbuzz".to_string();
        } else if v % 3 == 0 {
            return "fizz".to_string();
        } else if v % 5 == 0{
            return "buzz".to_string();
        }

        return v.to_string();
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
