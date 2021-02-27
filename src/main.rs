use std::env;
use fizzbuzz::fizzy;

fn main() {
    let args: Vec<String> = env::args().collect();

    for a in args[1..].iter() {
        print!("{} ", a);
        let number: i32 = match a.parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                eprintln!("'{}' is not an integer", a);
                return;
            }
        };
        println!("'{}'", fizzy::fizzbuzz(number));
    }
    
}
