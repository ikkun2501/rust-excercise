//extern crate competitive;
//
//use competitive::util::read_line;

fn main() {
    let mut input: Vec<Vec<char>> = vec![];
    input.push("W........WW.".chars().collect());
    input.push(".WWW.....WWW".chars().collect());
    input.push("....WW...WW.".chars().collect());
    input.push(".........WW.".chars().collect());
    input.push(".........W..".chars().collect());
    input.push("..W......W..".chars().collect());
    input.push(".W.W.....WW.".chars().collect());
    input.push("W.W.W.....W.".chars().collect());
    input.push(".W.W......W.".chars().collect());
    input.push("..W.......W.".chars().collect());


    for n in &input {
        for n2 in n {
            print!("{}", n2);
        }
        println!();
    }

    input[0][0] = '.';
    println!();

    for n in &input {
        for n2 in n {
            print!("{}", n2);
        }
        println!();
    }
}

