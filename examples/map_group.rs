use std::collections::HashMap;
use file::map::Tile;

pub fn main() {
    let map = String::from(BASE_DIR) + "/" + MAP_DIR + "/" + "n0.map";
    println!("map file path: {}", map);
    test_map_group(&map);
}

const BASE_DIR: &str = "/Users/vinter/Dev/Mir2";
const MAP_DIR: &str = "map";

const DATA_DIR: &str = "data";

pub fn test_map_group(map_file: &str) {
    let map_info = file::map::read_map_file(map_file);
    let mut hash: HashMap<u64, u32> = HashMap::new();
    let mut wzx_hash: HashMap<u16, Vec<u32>> = HashMap::new();

    println!("map: {}X{}, len: {}", map_info.width, map_info.height, map_info.tiles.len());
    for x in map_info.tiles {
        put_map(&x, &mut hash, &mut wzx_hash);
    }

    println!("hash: {}", hash.len());
    println!("wzxHash: {}", wzx_hash.len());
}

pub fn put_map(tile: &Tile, hash: &mut HashMap<u64, u32>, wzx: &mut HashMap<u16, Vec<u32>>) {
    if tile.back & 0x7FFF > 0 {
        // println!("back: {}, {}", tile.back, ((tile.back as u32) & 0x7FFF));
        // put_map0(hash, wzx, tile.tile_idx as u16 + 1, 1, ((tile.back as u32) & 0x7FFF) - 1);
    }
    if tile.middle & 0x7FFF > 0 {
        // println!("middle: {}, {}", tile.middle, ((tile.middle as u32) & 0x7FFF));
        put_map0(hash, wzx, tile.middle_idx as u16 + 1, 2, ((tile.middle as u32) & 0x7FFF) - 1);
    }
    if tile.objects & 0x7FFF > 0 {
        // put_map0(hash, wzx, tile.file_idx as u16 + 1, 3, ((tile.objects as u32) & 0x7FFF) - 1);
    }
}

fn put_map0(hash: &mut HashMap<u64, u32>, wzx: &mut HashMap<u16, Vec<u32>>, file: u16, tp: u16, idx: u32) {
    let wzx_key = file | (tp << 8);
    if !wzx.contains_key(&wzx_key) {
        let path = String::from(BASE_DIR) + "/" + DATA_DIR + "/";
        let file = if tp == 1 {
            convert_file_name(path.as_str(), "tiles", file as u8, "wzx")
        } else if tp == 2 {
            convert_file_name(path.as_str(), "smTiles", file as u8, "wzx") }
        else {
            convert_file_name(path.as_str(), "objects", file as u8, "wzx")
        };

        let wzx_vec = file::data::read_wzx(&file);
        wzx.insert(wzx_key, wzx_vec);
    }

    let wzx_vec = wzx.get(&wzx_key).unwrap();
    if idx as usize >= wzx_vec.len() {
        return;
    }
    // println!("file: {}, ty: {}, idx: {}", file, tp, idx);
    let view = wzx_vec[idx as usize];

    let key = ((wzx_key as u64) << 32) | view as u64;
    if !hash.contains_key(&key) {
        hash.insert(key, 1);
    } else {
        let v = hash.get(&key).unwrap();
        hash.insert(key, *v + 1);
    }
}

fn convert_file_name(dir: &str, name: &str, file: u8, suffix: &str) -> String {
    if file <= 1 {
        format!("{}{}.{}", dir, name, suffix)
    } else {
        format!("{}{}{}.{}", dir, name, file, suffix)
    }
}