extern crate rust_fizz_buzz;

use rust_fizz_buzz::fizzbuzz;


fn main() {
    for x in 1..101 {
        println!("{}",fizzbuzz::fizzbuzz(x));
    }
}

