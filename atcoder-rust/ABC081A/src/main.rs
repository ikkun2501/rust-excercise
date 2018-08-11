use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let x: Vec<String> = read_line();
    let s = &x[0];
    let count = s.chars().filter(|&c| c == '1').count();
    println!("{}", count);
}

fn read_line<T>() -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}
