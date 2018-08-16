use std::rc::Rc;

pub struct SpiderRobot {
    species: String,
    web_enabled: bool,
    leg_devices: [fd::FileDesc; 8],
}

pub struct SpiderSenses {
    robot: Rc<SpiderRobot>,
    eyes: [Camera; 32],
    motion: Accelerometer,
}

fn main() {
    println!("Hello, world!");
}
