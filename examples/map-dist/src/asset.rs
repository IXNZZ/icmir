use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use file::data::ImageData;
use file::map::{MapInfo, Tile};
use ggez::Context;
use ggez::glam::vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Image, ImageFormat, ScreenImage, Text, TextLayout};
use tracing::{debug, warn};

pub struct MapAsset {
    pub base_dir: String,
    pub map_file_name: String,
    pub map_info: MapInfo,
    pub x_point: u32,
    pub y_point: u32,
    pub image: ImageAsset,
    pub window_width: u32,
    pub window_height: u32,
    pub max_x_tile: u32,
    pub max_y_tile: u32,
    pub back_image: Option<Image>,
    pub sm_image: Option<Image>,
    pub obj_image: Option<Image>,
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
                    warn!("idx:{}, len: {}, key: {}", idx, index.len(), key);
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
    pub fn new(dir: &str, file: &str, width: u32, height: u32) -> Self {

        let info = file::map::read_map_file(String::from(dir.to_string() + file).as_str());
        let mut this = Self {
            base_dir: dir.to_string(),
            map_file_name: file.to_string(),
            map_info: info,
            x_point: 0,
            y_point: 0,
            max_x_tile: 0,
            max_y_tile: 0,
            window_width: width,
            window_height: height,
            image: ImageAsset::new(String::from(dir.to_string() + "data/").as_str()),
            back_image: None,
            sm_image: None,
            obj_image: None,
        };
        this.resize(width, height);
        this
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;
        self.max_x_tile = width / 48 + 1;
        self.max_y_tile = height / 32 + 1;

    }

    pub fn reload(&mut self, name: &str, x: u32, y: u32, ctx: &mut Context) {

        self.x_point = x;
        self.y_point = y;
        self.map_file_name = name.to_string();

        self.map_info = file::map::read_map_file(String::from(self.base_dir.clone() + name).as_str());

        let start_x = x;
        let start_y = y;
        // let start_x = if x > self.map_info.width { self.map_info.width } else { x };
        // let start_y = if y > self.map_info.height { self.map_info.height } else { y };

        let start_x = start_x as i32 - self.max_x_tile as i32 / 2 + 1;
        let start_y = start_y as i32 - self.max_y_tile as i32 / 2;

        let end_x = start_x + self.max_x_tile as i32;
        let end_y = start_y + self.max_y_tile as i32 + 10;

        // let mut tile_screen = ScreenImage::new(ctx, None, 1.0, 1.0, 1);
        // let mut middle_screen = ScreenImage::new(ctx, None, 1.0, 1.0, 1);
        let mut objects_screen = ScreenImage::new(ctx, None, 1.0, 1.0, 1);
        let back_image = objects_screen.image(ctx);
        let middle_image = objects_screen.image(ctx);
        let objects_image = objects_screen.image(ctx);
        let mut back_canvas = Canvas::from_image(ctx, back_image.clone(), None);
        let mut sm_canvas = Canvas::from_image(ctx, middle_image.clone(), None);
        let mut obj_canvas = Canvas::from_image(ctx, objects_image.clone(), None);

        let start_count = start_x * self.map_info.height as i32 + start_y;
        debug!("start_count: {}, map width: {}, height: {}", start_count, self.map_info.width, self.map_info.height);

        // let count = (end_x - start_x) * (end_y - start_y);

        let x_point = end_x - start_x;
        let y_point = end_y - start_y;

        debug!("size: {}, x_point: {}, y_point: {}, start_x: {}, start_y:{}", self.map_info.tiles.len(), x_point, y_point, start_x, start_y);


        for x in 0..x_point {
            for y in 0..y_point {
                let idx = x * self.map_info.height as i32 + y + start_count;
                if x + start_x < 0
                    || y + start_y < 0
                    || x + start_x >= self.map_info.width as i32
                    || y + start_y >= self.map_info.height as i32 { continue }
                self.load_image(x - 1, y, idx as usize, &mut back_canvas, &mut sm_canvas, &mut obj_canvas, ctx);
            }
        }
        {
            back_canvas.finish(ctx).unwrap();
            sm_canvas.finish(ctx).unwrap();
            obj_canvas.finish(ctx).unwrap();
        }

        let obj_screen = ScreenImage::new(ctx, None, 1.0, 1.0, 1).image(ctx);
        let mut objects_canvas = Canvas::from_image(ctx, obj_screen.clone(), None);
        // objects_canvas.draw(&back_image, DrawParam::new().dest(vec2(0.0, 0.0)));
        // objects_canvas.draw(&middle_image, DrawParam::new().dest(vec2(0.0, 0.0)));
        objects_canvas.draw(&objects_image, DrawParam::new().dest(vec2(0.0, 0.0)));

        objects_canvas.finish(ctx).unwrap();

        // self.back_image = Some(back_image);
        // self.sm_image = Some(middle_image);
        self.obj_image = Some(obj_screen);
    }

    fn load_image(&mut self, x: i32, y: i32, idx: usize, back_canvas: &mut Canvas, sm_canvas: &mut Canvas, obj_canvas: &mut Canvas, ctx: &mut Context) {
        let tile = &self.map_info.tiles[idx].clone();
        let ann = tile.frame > 0;
        let back = tile.back;
        let middle = tile.middle;
        let objects = tile.objects;

        // debug!("back: x: {:03}, y: {:03}, idx: {:05}, {:?}", x, y, idx, tile);
        if back & 0x7FFF > 0 && idx & 0x01 != 1 && (idx / self.map_info.height as usize) & 0x01 != 1 {

            let tile_idx = if tile.tile_idx != 0 { tile.tile_idx + 1 } else { 0 };

            // debug!("back: x: {:03}, y: {:03}, idx: {:05}, {:?}", x, y, idx, tile);
            self.draw_image(x, y + 1, "tiles", tile_idx, (back as u32 & 0x7FFF) - 1, back_canvas, ctx);
        }
        if middle & 0x7FFF > 0 {
            // if middle < 50000 {

            let middle = (middle as u32 & 0x7FFF) - 1;
            // let file_idx = if middle > 5000 && middle < 6000 { 5 } else { 3 };
            // // let file_idx = if middle > 1000 && middle < 1560 { 4 } else { file_idx };
            // let file_idx = if middle > 1000 && middle < 4000 { 5 } else { file_idx };
            // if middle < 1560 && tile.reserved != 0 { 4 }
            let middle_idx = if tile.middle_idx != 0 { tile.middle_idx + 1 } else { 0 };

            // debug!("middle: x: {:03}, y: {:03}, idx: {:05}, file: {}, {:?}", x, y, idx, file_idx, tile);
            self.draw_image(x, y, "smTiles", middle_idx, middle, sm_canvas, ctx);
            // }
        }
        if objects & 0x7FFF > 0 {
            let file_idx = if tile.file_idx > 0 && tile.file_idx < 255 { tile.file_idx + 1 } else { 0};
            // let file_idx = if file_idx > 10 && file_idx <= 19 {file_idx - 1} else { file_idx };
            self.draw_image(x, y, "objects", file_idx , (objects as u32 & 0x7FFF) -1, obj_canvas, ctx);
            if !ann  {
                // debug!("back: x: {:03}, y: {:03}, idx: {:05}, file: {}, {:?}", x, y, idx, file_idx, tile);
            } else {
                // debug!("back: x: {:03}, y: {:03}, idx: {:05}, file: {}, {:?}", x, y, idx, file_idx, tile);
            }
            // debug!("objects: x: {:03}, y: {:03}, idx: {:05}, {:?}", x, y, idx, tile);

        }
    }

    fn draw_image(&mut self, x: i32, y: i32, name: &str, file_idx: u8, image_idx: u32, canvas: &mut Canvas, ctx: &mut Context) {
        if let Some(image) = self.image.load_image_asset(name, file_idx, image_idx) {
            if image.bytes.len() > 0 {
                let img = Image::from_pixels(ctx, &image.bytes[..],
                                             ImageFormat::Rgba8UnormSrgb,
                                             image.width as u32,
                                             image.height as u32);
                let dest = vec2(x as f32 * 48., y as f32 * 32.0 - image.height as f32);
                // debug!("image: x:{}, y:{}, name:{}, idx: {}, offsetX: {}, offsetY: {}, w: {}, h: {}", x, y, name, image_idx, x as f32 * 48., y as f32 * 32.0 + 32.0 - image.height as f32, image.width, image.height);
                canvas.draw(&img, DrawParam::new().dest(dest));
                // canvas.draw(Text::new(format!(":{}\n{}", image_idx, file_idx)).set_scale(14.0), dest);
            }
        }
    }

    pub fn jump(&mut self, x: u32, y: u32, ctx: &mut Context) {

    }

    pub fn move_pixel(&mut self, x: i32, y: i32, ctx: &mut Context) {

    }

}



