fn main() {
    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    println!("{},{}", point.x, point.y);
    println!("{},{}", r.x, r.y);
    println!("{},{}", (*r).x, (*r).y);
    println!("{},{}", (**rr).x, (**rr).y);
    println!("{},{}", (***rrr).x, (***rrr).y);
    // compile error
    // println!("{},{}", (****rrr).x, (****rrr).y);
}
