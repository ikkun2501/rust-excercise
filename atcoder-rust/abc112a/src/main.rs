fn main() {
    fn read_line() -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().to_string()
    }

    let n: i32 = read_line().parse().unwrap();

    if n == 1 {
        println!("Hello World");
    } else {
        let a: i32 = read_line().parse().unwrap();
        let b: i32 = read_line().parse().unwrap();
        println!("{}", a + b);
    }
}

