static mut STASH: &i32 = &10;
fn f(p: &'static i32){
    unsafe{
        STASH=p;
    }
}

fn main() {
    println!("Hello, world!");
}
