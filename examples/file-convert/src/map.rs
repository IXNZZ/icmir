use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use bytes::Buf;
use image::{RgbaImage};
use file::data::ImageData;
use file::map;
use file::map::MapInfo;
use image::imageops::FilterType;
use tokio::sync::Semaphore;
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

pub fn test_map() {
    let path = Path::new(config::BASE_DIR).join(config::MAP_DIR_NAME).join( "n0.map");
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
    // let mut tiles = Vec::with_capacity((width * height) as usize);
    for i in 187921..189000 {
        let start = i as usize * length;
        let end = start + length;
        // let tile = Tile::from(&body[start..end]);
        // tiles.push(tile);
        println_hex(&body[start..end], length, i);
    }
}

pub fn println_hex(src: &[u8], length: usize, idx: u32) {
    println!("{}===>{:02X?}", idx, src)
}



pub struct MapAsset {
    pub base_dir: String,
    pub image: ImageAsset
}

pub struct ImageAsset {
    dir: String,
    pub image: HashMap<u64, ImageData>,
    pub index: HashMap<String, Vec<u32>>,
}

impl ImageAsset {

    pub fn new(dir: &str) -> Self {
        Self {dir: dir.to_string(), image: HashMap::new(), index: HashMap::new()}
    }
    pub fn load_image_asset(&mut self, name: &str, file: u8, idx: u32) -> Option<&ImageData> {
        let key = self.convert_file_name(self.dir.as_str(), name, file, "wzx");
        let f = self.convert_file_name(self.dir.as_str(), name, file, "wzl");

        if !self.index.contains_key(&key) {
            let index = file::data::read_wzx(key.as_str());
            self.index.insert(key.clone(), index);
        }
        if let Some(index) = self.index.get(key.as_str()) {
            if idx as usize >= index.len() {
                // warn!("idx:{}, len: {}, key: {}", idx, index.len(), key);
                return None;
            }
            let i = index[idx as usize];
            let h = name.as_bytes()[0] as u64;
            let c = file as u64;
            let k = h << 40 | c << 32 | i as u64;
            if !self.image.contains_key(&k) {
                // debug!("image:{} idx: {}, file: {}, key: {}", idx, i, f, key);
                let image_data = file::data::load_image(f.as_str(), i, i + 16);
                self.image.insert(k, image_data);
                // return Some(image_data);
                //self.image.insert(key.clone(), image_data);
            }
            return self.image.get(&k);
        }

        None
    }

    fn convert_file_name(&self, dir: &str, name: &str, file: u8, suffix: &str) -> String {
        if file <= 1 {
            format!("{}{}.{}", dir, name, suffix)
        } else {
            format!("{}{}{}.{}", dir, name, file, suffix)
        }
    }
}

impl MapAsset {
    pub fn new(dir: &str) -> Self {
        let mut this = Self {
            base_dir: String::from(dir.to_string()),
            image: ImageAsset::new(String::from(dir.to_string() + "/data/").as_str())
        };

        this
    }

    pub async fn save_all() {
        let dir = Path::new(config::BASE_DIR).join(config::MAP_DIR_NAME).read_dir().unwrap();
        let data_dir = config::BASE_DIR.to_string() + "/data/";
        let mut files:Vec<String> = dir.map(|x| {
            String::from(x.unwrap().path().to_str().unwrap())
        }).filter(|x| { x.ends_with(".map") }).collect();
        files.sort();
        let semaphore = Arc::new(Semaphore::new(2));
        for file in files {
            let semaphore = semaphore.clone();
            // let _permit = semaphore.acquire_owned().await.unwrap();
            // tokio::spawn(async move {
            let info = map::read_map_file(file.as_str());
            let output = format!("{}/save/{}_{}_{}.webp", config::BASE_DIR, info.name, info.width, info.height);
            if !Path::new(&output).exists() {
                // if info.width <= 300 && info.height <= 300 {
                    println!("read file: {}", file);
                    MapAsset::new(config::BASE_DIR).save_info(info, semaphore).await;
                // }
                // MapAsset::new(config::BASE_DIR).save_info(info, semaphore).await;
            } else {
                println!("file exists: {}", output);
            }
                // if info.width > 340 || info.height > 510 {
                //     // eprintln!("Ignore file: {}, {}X{}", file, info.width, info.height);
                // } else {
                //     MapAsset::new(config::BASE_DIR).save_info(info, semaphore).await;
                // }
                // drop(_permit);
            // });
        }

    }


    pub async fn save(&mut self, name: &str, semaphore: Arc<Semaphore>) {
        let map_info = file::map::read_map_file(String::from(self.base_dir.clone() + "/map/" + name).as_str());
        self.save_info(map_info, semaphore).await;
    }

    pub async fn save_info(&mut self, map_info: MapInfo, semaphore: Arc<Semaphore>) {
        let now = Instant::now();
        // let map_info = file::map::read_map_file(String::from(self.base_dir.clone() + "/map/" + name).as_str());

        let start_x = 0;
        let start_y = 0;
        let end_x = start_x + map_info.width as i32;
        let end_y = start_y + map_info.height as i32;

        let x_point = end_x - start_x;
        let y_point = end_y - start_y;
        let mut rgba_image = RgbaImage::new(map_info.width * 48, map_info.height * 32);
        // println!("startX: {}, startY: {}, endX: {}, endY: {}, now: {:?}", start_x, start_y, end_x, end_y, now.elapsed().as_millis());
        for x in 0..x_point {
            for y in 0..y_point {
                let idx = x * map_info.height as i32 + y;
                if x + start_x < 0
                    || y + start_y < 0
                    || x + start_x >= map_info.width as i32
                    || y + start_y >= map_info.height as i32 { continue }
                self.load_image(x, y + 1, idx as usize, &map_info, &mut rgba_image);
            }
        }
        // let output = self.base_dir.clone() + "/save/" + name + ".webp";
        let output = format!("{}/save/{}_{}_{}.webp", self.base_dir, map_info.name, map_info.width, map_info.height);
        // let output = if end_x > 340 || end_y > 510 { format!("{}.png", output) } else { format!("{}.webp", output) };
        println!("load image now: {:04?}, output: {}", now.elapsed().as_millis(), output);
        let permit = semaphore.acquire_owned().await.unwrap();
        tokio::spawn(async move{
            let now = Instant::now();
            MapAsset::save_image(rgba_image, map_info.width * 48, map_info.height * 32, output.as_str());
            println!("save finish: {}, map: {:?}", map_info.name, now.elapsed().as_millis());
            drop(permit);
        });


        // MapAsset::save_image(rgba_image, map_info.width * 48, map_info.height * 32, output.as_str());

        // println!("save finish: {:?}", now.elapsed().as_millis());
    }

    fn save_image(image: RgbaImage, width: u32, height: u32, output: &str) {
        if width >= 0x3FFF || height >= 0x3FFF {
            MapAsset::save_image(image, width / 2, height / 2, output);
            return;
        }
        if image.width() != width || image.height() != height {
            let now = Instant::now();
            let dest = image::imageops::resize(&image, width, height, FilterType::Triangle);
            println!("resize image: {}X{} now:{:?}", dest.width(), dest.height(), now.elapsed().as_millis());
            dest.save(output).unwrap();
            // libwebp_sys::WebEn
        } else {
            image.save(output).unwrap();
        }

    }

    fn load_image(&mut self, x: i32, y: i32, idx: usize, map_info: &MapInfo, rgba: &mut RgbaImage) {
        let tile = &map_info.tiles[idx];
        let back = tile.back;
        let middle = tile.middle;
        let objects = tile.objects;

        if back & 0x7FFF > 0 && idx & 0x01 != 1 && (idx / map_info.height as usize) & 0x01 != 1 {
            let tile_idx = if tile.tile_idx != 0 && tile.tile_idx < 22 { tile.tile_idx + 1 } else { 0 };

            self.draw_image(x, y + 1, "tiles", tile_idx, (back as u32 & 0x7FFF) - 1, rgba);
        }
        if middle & 0x7FFF > 0 {

            let middle = (middle as u32 & 0x7FFF) - 1;
            let middle_idx = if tile.middle_idx != 0 && tile.middle_idx < 36 { tile.middle_idx + 1 } else { 0 };

            // debug!("middle: x: {:03}, y: {:03}, idx: {:05}, file: {}, {:?}", x, y, idx, file_idx, tile);
            self.draw_image(x, y, "smTiles", middle_idx, middle, rgba);
            // }
        }
        if objects & 0x7FFF > 0 {
            let file_idx = if tile.file_idx > 0 && tile.file_idx < 51 { tile.file_idx + 1 } else { 0};
            // let file_idx = if file_idx > 10 && file_idx <= 19 {file_idx - 1} else { file_idx };
            if tile.frame == 0  {
                self.draw_image(x, y, "objects", file_idx , (objects as u32 & 0x7FFF) -1, rgba);
                // debug!("back: x: {:03}, y: {:03}, idx: {:05}, file: {}, {:?}", x, y, idx, file_idx, tile);
            }
            // debug!("objects: x: {:03}, y: {:03}, idx: {:05}, {:?}", x, y, idx, tile);

        }
    }

    fn draw_image(&mut self, x: i32, y: i32, name: &str, file_idx: u8, image_idx: u32, dest: &mut RgbaImage) {
        if let Some(image) = self.image.load_image_asset(name, file_idx, image_idx) {
            if image.bytes.len() > 0 {
                if let Some(rgb) = RgbaImage::from_raw(image.width as u32, image.height as u32, image.bytes.to_vec()) {
                    // println!("x: {}, y: {}", x as i64 * 48, y as i64 * 32 - image.height as i64);
                    image::imageops::overlay(dest, &rgb, x as i64 * 48, y as i64 * 32 - image.height as i64)
                    // image::imageops::overlay()
                }

                // dest.
                // RgbaImage::from_pixel(image.width as u32, image.height as u32)
                // let img = Image::from_pixels(ctx, &image.bytes[..],
                //                              ImageFormat::Rgba8UnormSrgb,
                //                              image.width as u32,
                //                              image.height as u32);
                // let dest = vec2(x as f32 * 48., y as f32 * 32.0 - image.height as f32);
                // // debug!("image: x:{}, y:{}, name:{}, idx: {}, offsetX: {}, offsetY: {}, w: {}, h: {}", x, y, name, image_idx, x as f32 * 48., y as f32 * 32.0 + 32.0 - image.height as f32, image.width, image.height);
                // canvas.draw(&img, DrawParam::new().dest(dest));
                // canvas.draw(Text::new(format!(":{}\n{}", image_idx, file_idx)).set_scale(14.0), dest);
            }
        }
    }

}





