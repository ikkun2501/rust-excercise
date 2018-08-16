extern crate competitive;

use competitive::util::read_line;

fn fact(n: u32) -> u32 {
    if n == 0 { 1 } else { n * fact(n - 1) }
}

fn main() {
    println!("{}", fact(read_line()[0]));
}

