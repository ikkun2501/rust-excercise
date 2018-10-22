use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let x: Vec<u32> = read_line();
    let num1 = x[0];
    let num2 = x[1];
    let product = num1 * num2;
    match product {
        product if product % 2 == 0 => println!("Even"),
        _ => println!("Odd")
    }
}


fn read_line<T>() -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}