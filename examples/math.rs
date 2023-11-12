use std::f32::consts::PI;
use file::asset::{FileDesc, FileDescType};

pub fn main() {
    let x1 = 0.0_f32;
    let y1 = 0.0_f32;
    let x2 = 1.0_f32;
    let y2 = 0.0_f32;

    let x = (x2 - x1).abs().powf(2.0);
    let y = (y2 - y1).abs().powf(2.0);
    let z = (x + y).sqrt();
    println!("x: {x}, y: {y}, z: {z}");
    let x3 = (x / y).atan() * 180.0 / PI;
    println!("x3: {x3}");

    let point_x = Point::new(0.0, 0.0);
    println!("(1, 0) angle: {}", angle(&point_x, &Point::new(1.0, 0.0)));
    println!("(1, 1) angle: {}", angle(&point_x, &Point::new(1.0, 1.0)));
    println!("(0, 1) angle: {}", angle(&point_x, &Point::new(0.0, 2.0)));
    println!("(-1, 1) angle: {}", angle(&point_x, &Point::new(-1.0, 1.0)));
    println!("(-1, 0) angle: {}", angle(&point_x, &Point::new(-1.0, 0.0)));
    println!("(-1, -1) angle: {}", angle(&point_x, &Point::new(-1.0, -1.0)));
    println!("(0, -1) angle: {}", angle(&point_x, &Point::new(0.0, -1.0)));
    println!("(1, -1) angle: {}", angle(&point_x, &Point::new(1.0, -1.0)));

    let mut asset = file::asset::create_default_image_asset("/Users/vt/Documents/LegendOfMir");

    asset.put_file_map(5, "hair", true);
    let data = asset.load_image(FileDesc::ZONE {file: 5, number: 0, index: 833}, FileDescType::IDX).unwrap();

}

pub fn angle(p1: &Point, p2: &Point) -> f32 {
    // let x = (p1.x - p2.x).abs().powi(2);
    // let y = (p1.y - p2.y).abs().powi(2);
    // (y / (x + y).sqrt()).asin() / PI * 180.0
    (p2.y - p1.y).atan2(p2.x - p1.x) * 180.0 / PI
}

pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}