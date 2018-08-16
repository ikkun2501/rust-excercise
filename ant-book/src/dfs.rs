extern crate competitive;

use competitive::util::read_line;

fn main() {
    let _n: usize = read_line()[0];
    let a: Vec<u32> = read_line();
    let k: u32 = read_line()[0];

    if dfs(0, 0, &a, k) { println!("Yes!") } else { println!("No!") }
    if dfs(0, 0, &vec![1, 2, 4, 7], 13) { println!("Yes!") } else { println!("No!") }
    if dfs(0, 0, &vec![1, 2, 4, 7], 15) { println!("Yes!") } else { println!("No!") }
}

fn dfs(i: usize, sum: u32, a: &Vec<u32>, k: u32) -> bool {
    if i == a.len() { return sum == k; }

    if dfs(i + 1, sum, a, k) { return true; }


    if dfs(i + 1, sum + a[i], a, k) { return true; }

    false
}

