//関数に引き渡すときは構造体の複製はおこzラズに、単にポインタの複製を行っているだけである。
// いわば、暗黙の参照渡しであるが、所有権を手放してしまう。

struct Point {
    x: i32,
    y: i32,
}

fn func(v: Point){
    println!("{},{}",v.x,v.y);
}

fn main() {
    let a = Point { x: 100, y: 230 };
    func(a);
    println!("{},{}", a.x, a.y);
}
