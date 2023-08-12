use std::fmt::format;
use std::fs::File;
use std::io::{BufReader, Read, sink};
use std::path::Path;
use bytes::{Buf, Bytes};
use ggez::{Context, GameError, GameResult, graphics};
use ggez::event::{EventHandler, ScanCode};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, InstanceArray, Quad, Rect};
use ggez::input::keyboard::{KeyCode, KeyInput};
use crate::file::MapInfo;

pub struct App{
    index: usize,
    size: usize,
    offset_x: usize,
    offset_y: usize,
    files: Vec<String>,
    array: InstanceArray,
    title: String,
    map: Option<crate::file::MapInfo>
}

impl App {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let dir = Path::new("/Users/vinter/Dev/Mir2/Map");
        let files: Vec<String> = dir.read_dir()?.map(|x| { String::from(x.unwrap().path().as_os_str().to_str().unwrap()) })
            .filter(|x| { x.ends_with(".map") }).collect();
        let mut array = InstanceArray::new(_ctx, None);
        let index = 0;
        let size = files.len();
        Ok(App {index, size, offset_x: 50, offset_y: 50, files, array, title: "None".to_string(), map: None})
    }

    pub fn load_tile(&mut self, ctx: &mut Context, idx: usize) {
        if idx >= self.size {
            self.title = format!("idx: {}, size: {}", idx, self.size);
            return;
        }

        let x = self.files.get(idx).unwrap();
        let map = crate::file::read_map_file(x);
        self.title = format!("idx: {}, w: {}, h: {}, step: {}, size: {}, name: {}", self.index, map.width, map.height, map.step, map.size, map.name.clone());
        // self.map = Some(map);
        self.array.clear();
        self.array.resize(ctx, (map.width * map.height) as usize);
        // self.array.set(0..map.height)
        for i in 0..map.tiles.len() {
            let x = i as u32 / map.height;
            let y = i as u32 % map.height;
            let tile = map.tiles.get(i).unwrap();
            if tile.back & 0x7FFF > 0 {
                let dp = DrawParam::new().dest_rect(Rect::new_i32(x as i32, y as i32, 1, 1)).color(Color::RED);
                self.array.push(dp);
            }
        }
        self.map = Some(map);
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {

        let mut canvas = Canvas::from_frame(ctx, Color::new(0.1, 0.2, 0.3, 1.0));
        let dest_point = Vec2::new(0., 0.);
        canvas.draw(
            graphics::Text::new(self.title.as_str())
                .set_scale(32.),
            dest_point,
        );
        canvas.draw(&self.array, DrawParam::new().dest(Vec2::new(self.offset_x as f32, self.offset_y as f32)));
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        // input {  }
        match input.keycode {
            Some(KeyCode::Up) => {
                if self.index > 0 {
                    self.index -=1;
                    self.load_tile(ctx, self.index);
                }
            },
            Some(KeyCode::Down) => {
                if self.index < self.size {
                    self.index += 1;
                    self.load_tile(ctx, self.index);
                }
            }
            _ => {

            }
        }

        Ok(())
    }
}

