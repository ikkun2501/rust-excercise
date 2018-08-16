struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

#[test]
fn test_new_map() {
    let image = new_map((1024, 576), vec![0; 1024 * 576]);
    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }

fn chop(b: Broom) -> (Broom, Broom) {
    // broom1の大半をbから作り高さだけを半分にする
    // String は Copyではないので、broom1はbの名前の所有権を得る
    let mut broom1 = Broom { height: b.height / 2, ..b };
    // その他フィールドをコピー ..broom1
    let mut broom2 = Broom { name: broom1.name.clone(), ..broom1 };

    broom1.name.push_str(" 1");
    broom2.name.push_str(" 2");

    (broom1, broom2)
}

#[test]
fn test_chop() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };
    let (hokey1, hokey2) = chop(hokey);

    assert_eq!(hokey1.name, "Hokey 1");
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey 2");
    assert_eq!(hokey2.health, 100);
}

fn main() {
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
    println!("Hello, world!");
}


