extern crate rust_fizz_buzz;

use rust_fizz_buzz::fizzbuzz;

#[test]
fn can_test() {
    assert_eq!(1, 1);
}

#[test]
fn _1is_1() {
    assert_eq!("1", fizzbuzz::fizzbuzz(1));
}

#[test]
fn _3is_fizz() {
    assert_eq!("fizz", fizzbuzz::fizzbuzz(3));
}

#[test]
fn _5is_buzz() {
    assert_eq!("buzz", fizzbuzz::fizzbuzz(5));
}

#[test]
fn _15is_fizzbuzz() {
    assert_eq!("fizzbuzz", fizzbuzz::fizzbuzz(15));
}