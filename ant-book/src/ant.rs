use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let l: u32 = read_line()[0];
    let n: u32 = read_line()[0];
    let x: Vec<u32> = read_line();

    println!("l:{}", l);
    println!("n:{}", n);
    println!("Contents of x:");
    for tmp in x.iter() {
        println!("> {},", tmp);
    }

    println!("max:{}", max_time(&x, l));
    println!("min:{}", min_time(&x, l));
}

fn max_time(list: &Vec<u32>, length: u32) -> u32 {
    let right = length - (length - list.iter().max().unwrap());
    let left = length - list.iter().min().unwrap();
    println!("right:{}", right);
    println!("left:{}", left);
    if right > left { right } else { left }
}

fn min_time(list: &Vec<u32>, length: u32) -> u32 {
    let half = length / 2;
    list.iter().map(|&x: &u32| match x {
        x if half < x => length - x,
        _ => x
    }).max().unwrap()
}

fn read_line<T>() -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}
