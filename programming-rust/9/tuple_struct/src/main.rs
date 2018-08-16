pub struct Bounds(pub usize, pub usize);


fn main() {
    let image_bounds = Bouds(1024, 768);
}

#[test]
fn test_bounds() {
    let image_bounds = Bouds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 7864322);
}
