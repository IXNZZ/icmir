use std::collections::HashMap;
use std::ffi::OsString;
use std::io::{Read, stdin};
use std::path::Path;
use file::map::Tile;

pub fn main() {
    let map = String::from(BASE_DIR) + "/" + MAP_DIR + "/";

    let path = Path::new(&map);
    let mut path: Vec<OsString> = path.read_dir().unwrap().map(|x| {
        let x = x.unwrap().file_name();
        //x.to_str().unwrap()
        x.to_ascii_lowercase()
    }).filter(|x| {
        x.to_str().unwrap().ends_with(".map")
    }).collect();

    path.sort();

    path.iter().for_each(|x| {
        let x = x.to_str().unwrap();
        // println!("name: {}", x);
        test_map_group(&map, x);
    })

    // println!("map file path: {}", map);
    // test_map_group(&map);
}

const BASE_DIR: &str = "/Users/vt/Documents/LegendOfMir";
const MAP_DIR: &str = "map";

const DATA_DIR: &str = "data";

pub fn test_map_group(base_dir: &str, map_file: &str) {
    let file = String::from(base_dir) + map_file;
    let map_info = file::map::read_map_file(&file);
    let mut hash: HashMap<u64, u32> = HashMap::new();
    let mut wzx_hash: HashMap<u16, Vec<u32>> = HashMap::new();

    let mut idx = 0;
    for x in &map_info.tiles {
        // if idx & 0x01 != 1 && (idx / map_info.height as usize) & 0x01 != 1 {
            put_map(x, &mut hash, &mut wzx_hash);
        // }
        idx += 1;
    }

    let mut back = 0;
    let mut middle = 0;
    let mut objects = 0;
    let mut back_value = 0;
    let mut middle_value = 0;
    let mut objects_value = 0;
    hash.iter().for_each(|(k, v)| {

        let x = *k >> 40;
        if x == 1 {
            back +=1;
            back_value += *v;
        } else if x == 2 {
            middle +=1;
            middle_value += *v;
        } else {
            objects +=1;
            objects_value += *v;
        }
    });
    // hash.keys().for_each(|x| {
    //     let x = *x >> 40;
    //    if x == 1 {
    //        back +=1;
    //    } else if x == 2 {
    //        middle +=1;
    //    } else {
    //        objects +=1;
    //    }
    // });
    // println!("hash: {}", hash.len());
    // let len = map_info.tiles.len();
    let back_value_f = 100.0 - (back as f32 / back_value as f32) * 100.0;
    let middle_value_f = 100.0 - (middle as f32 / middle_value as f32) * 100.0;
    let objects_value_f = 100.0 - (objects as f32 / objects_value as f32) * 100.0;
    if back_value_f > 90.0 || middle_value_f > 90.0 || objects_value_f > 90.0 {
        println!("map: {:>4}X{:<4}, len: {:7}, name: {}", map_info.width, map_info.height, map_info.tiles.len(), map_info.name);
        println!("back_value: {}, middle_value: {}, objects_value: {}", back_value, middle_value, objects_value);
        println!("============================================================\
    back: {:>6}|{:.4}, middle: {:>6}|{:.4}, objects: {:>6}|{:.4}",
                 back, back_value_f,
                 middle, middle_value_f,
                 objects, objects_value_f);
    }

    let stdin1 = stdin();
    // stdin1.read()
    // println!("len: {}, data: {:?}", hash.len(), hash)
}

pub fn put_map(tile: &Tile, hash: &mut HashMap<u64, u32>, wzx: &mut HashMap<u16, Vec<u32>>) {
    if tile.back & 0x7FFF > 0 {
        // println!("back: {}, {}", tile.back, ((tile.back as u32) & 0x7FFF));
        put_map0(hash, wzx, tile.tile_idx as u16 + 1, 1, ((tile.back as u32) & 0x7FFF) - 1);
    }
    if tile.middle & 0x7FFF > 0 {
        // println!("middle: {}, {}", tile.middle, ((tile.middle as u32) & 0x7FFF));
        put_map0(hash, wzx, tile.middle_idx as u16 + 1, 2, ((tile.middle as u32) & 0x7FFF) - 1);
    }
    if tile.objects & 0x7FFF > 0 {
        put_map0(hash, wzx, tile.file_idx as u16 + 1, 3, ((tile.objects as u32) & 0x7FFF) - 1);
    }
}

fn put_map0(hash: &mut HashMap<u64, u32>, wzx: &mut HashMap<u16, Vec<u32>>, file: u16, tp: u16, idx: u32) {
    if file > 100 {
        return;
    }
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

    if view == 48 {
        return;
    }

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