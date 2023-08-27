use crate::map::MapAsset;

mod data;
mod config;
mod map;

fn main() {
    // let x = include_bytes!("/Users/vinter/Dev/Mir2/data/Hum.wzl");

    // data::convert_data();
    // map::check_map();
    // map::test_map();
    let mut map = MapAsset::new(config::BASE_DIR);
    map.save("ygfx1.map");
}
