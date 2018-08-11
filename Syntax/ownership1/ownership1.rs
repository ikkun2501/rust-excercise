// Rustでは１つの値についての所有権を複数の変数が同時に持つことはできない
fn main() {
    let v = vec![1, 2, 3];

    let v2 = v;

    println!("v[0] is: {}", v[0]);
}