extern crate competitive;

use competitive::util::read_line;

fn fib(n: u32) -> u32 {
    match n {
        n if n== 1 => 1,
        n if n== 0 => 0,
        _ => fib(n-1) + fib(n-2)
    }
}

fn main() {
    println!("{}", fib(read_line()[0]));
}
