
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use bytes::Buf;

pub fn read_map_file(path: &str) -> MapInfo {
    let path = Path::new(path);
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let file_size = path.metadata().unwrap().len();
    let mut file = File::open(path).unwrap();
    let mut header = [0u8; 52];
    file.read(&mut header).unwrap();
    let mut header = &header[..];
    let width = header.get_u16_le() as u32;
    let height = header.get_u16_le() as u32;
    let length = ((file_size as u32 - 52) / (width * height)) as usize;
    let mut body = Vec::with_capacity(file_size as usize -52);
    file.read_to_end(&mut body).unwrap();
    // let mut reader = BufReader::new(file);
    let mut tiles = Vec::with_capacity((width * height) as usize);
    for i in 0..width * height {
        let start = i as usize * length;
        let end = start + length;
        let tile = Tile::from(&body[start..end]);
        tiles.push(tile);
    }
    MapInfo {width, height, step: length as u32, size: file_size, name, tiles}
}

pub struct MapInfo {
    pub width: u32,
    pub height: u32,
    pub step: u32,
    pub size: u64,
    pub name: String,
    pub tiles: Vec<Tile>
}

#[derive(Debug)]
pub struct Tile {
    pub back: u16,
    pub middle: u16,
    pub objects: u16,
    pub door_idx: u8,
    pub door_offset: u8,
    pub frame: u8,
    pub tick: u8,
    pub file_idx: u8,
    pub light: u8,
    pub reserved: u8,
}

impl Tile {
    pub fn from(bytes: &[u8]) -> Self {
        let len = bytes.len();
        let mut bytes = bytes;
        let back = bytes.get_u16_le();
        let middle = bytes.get_u16_le();
        let objects = bytes.get_u16_le();
        let door_idx = bytes.get_u8();
        let door_offset = bytes.get_u8();
        let frame = bytes.get_u8();
        let tick = bytes.get_u8();
        let file_idx = bytes.get_u8();
        let light = bytes.get_u8();
        let mut reserved = 0;
        if len > 12 {
            reserved = bytes.get_u8();
        }
        Tile { back, middle, objects, door_idx, door_offset, frame, tick, file_idx, light, reserved }
    }
}