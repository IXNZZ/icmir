use std::collections::HashSet;
use std::path::Path;
use file::map;
use crate::config;

pub fn check_map() {
    let dir = Path::new(config::BASE_DIR).join(config::MAP_DIR_NAME).read_dir().unwrap();
    let mut files:Vec<String> = dir.map(|x| {
        String::from(x.unwrap().path().to_str().unwrap())
    }).filter(|x| { x.ends_with(".map") }).collect();
    files.sort();
    let mut file_idx = HashSet::new();
    for file in &files {
        let mut bg = HashSet::new();
        let mut mid = HashSet::new();
        let mut obj = HashSet::new();
        let mut check = false;
        let info = map::read_map_file(file.as_str());
        let mut num = 0;
        if info.tiles.len() > 0 {
            for tile in info.tiles {
                num += 1;
                if tile.back & 0x7FFF > 0 {
                    bg.insert(tile.back & 0x7FFF);
                }
                if tile.middle & 0x7FFF > 0 {
                    mid.insert(tile.middle & 0x7FFF);
                }
                if tile.objects & 0x7FFF > 0 {
                    obj.insert((tile.objects & 0x7FFF) as u32 | ((tile.file_idx as u32) << 16));
                    file_idx.insert(tile.file_idx);
                    if tile.file_idx > 31 && tile.file_idx == 255 {
                        check = true;
                        // println!("num: {}, h: {}, w: {}, tile: {:?}", num, num / info.height + 1, num % info.height, tile);
                    }
                }


            }
        }
        if check {
            println!("w: {:03}, h: {:03}, obj: {}, mid: {:02}, bg: {}, size: {:07}, step: {}, name: {}",
                     info.width, info.height, obj.len(), mid.len(), bg.len(), (info.size as u32 - 52) / info.step, info.step, info.name);
        }
    }
    println!("map files: {}, idx: {:?}", files.len(), file_idx);
}

pub fn load_wzx(name: &str) -> Vec<u32> {
    let path = Path::new(config::BASE_DIR).join(config::DATA_DIR_NAME).join(name.to_string() + ".wzx");
    crate::data::read_wzx(path.to_str().unwrap())
}

