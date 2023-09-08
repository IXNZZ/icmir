use ggez::event::EventHandler;
use ggez::{Context, GameError, GameResult};
use ggez::glam::vec2;
use ggez::graphics::{BlendMode, Canvas, Color, DrawMode, DrawParam, Rect, ScreenImage};
use ggez::input::keyboard::KeyInput;
use ggez::winit::event::VirtualKeyCode;
use image::{DynamicImage, RgbaImage};
use tracing::debug;
use crate::asset;
use crate::asset::MapAsset;

pub struct App {
    map_asset: asset::MapAsset,
    reload_map: bool,
    point_x: u32,
    point_y: u32,
}

impl App {

    pub fn new(ctx: &mut Context) -> Self {
        ctx.gfx.set_resizable(false).unwrap();
        ctx.gfx.set_drawable_size(1920.0, 1280.0).unwrap();
        let mut layer = ggez::graphics::ScreenImage::new(ctx, None, 1.0, 1.0, 1);
        // let mut canvas = ggez::graphics::Canvas::from_image(ctx, layer.image(ctx), None);
        let image = layer.image(ctx);
        println!("width: {}, height: {}", image.width(), image.height());
        // let image1 = DynamicImage::from(image.);
        // let raw = RgbaImage::from_raw(image.width(), image.height(), image.to_pixels(ctx).unwrap());
        // if let Some(x) = raw {
        //     x.save("/Users/vinter/Dev/raw.png");
        // }

        // file::data::load_image("/Users/vinter/Dev/Mir2/data/objects.wzl", 3427657, 3436955);
        // file::data::load_image("/Users/vinter/Dev/Mir2/data/objects.wzl", 3427657, 3427657 + 16);
        // /Users/vt/Documents/LegendOfMir/
        // 0x6D, 0x32 ASCII

        Self {map_asset: MapAsset::new("/Users/vinter/Dev/Mir2/", "map/0.map", 1920, 1280),
            reload_map: false, point_x: 320, point_y: 280}
    }
}

impl EventHandler<ggez::GameError> for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // if ctx.time.ticks() % 100 == 0 {
        //     println!("Delta frame time: {:?} ", ctx.time.delta());
        //     println!("Average FPS: {}", ctx.time.fps());
        //     debug!("Average ticks: {}", ctx.time.ticks());
        // }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(_ctx, Color::new(0., 0., 0., 1.0));
        if self.reload_map {
            self.map_asset.reload("map/n0.map", self.point_x, self.point_y, _ctx);
            self.reload_map = false;
        }

        if _ctx.time.ticks() % 2 == 0 {
            // return Ok(());
        }

        if let Some(image) = &self.map_asset.back_image {
            // println!("draw");
            canvas.draw(image, DrawParam::new().dest(vec2(0.0, 0.0)));
        }
        if let Some(image) = &self.map_asset.sm_image {
            canvas.draw(image, DrawParam::new().dest(vec2(0.0, 0.0)));
        }
        // canvas.set_blend_mode(BlendMode::ADD);
        if let Some(image) = &self.map_asset.obj_image {
            canvas.draw(image, DrawParam::new().dest(vec2(0.0, 0.0)));
        }
        // let circle = ggez::graphics::Mesh::new_circle(_ctx, DrawMode::fill(), vec2(100.0, 200.0), 50., 1., Color::WHITE);
        // canvas.draw(&circle.unwrap(), vec2(100., 1.));
        // canvas.finish(_ctx).unwrap();

        // let mut layer = ScreenImage::new(_ctx, None, 1.0, 1.0, 1);
        // let image = layer.image(_ctx);

        // let image1 = DynamicImage::from(image);

        // let mut canvas = Canvas::from_frame(_ctx, None);
        // let param = DrawParam::new().dest(vec2(50.0, 1.0));
        // canvas.draw(&image, param);
        _ctx.gfx.set_window_title(&format!(
            "Map Browser - {:.0} FPS, {}X{}",
            _ctx.time.fps(),
            self.point_x,
            self.point_y,
        ));
        canvas.finish(_ctx).unwrap();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {

        if let Some(x) = input.keycode {
            match x {
                VirtualKeyCode::R => {
                    self.reload_map = true;
                }
                VirtualKeyCode::Left => {
                    self.point_x -= 1;
                    self.reload_map = true;
                }
                VirtualKeyCode::Up => {
                    self.point_y -= 1;
                    self.reload_map = true;
                }
                VirtualKeyCode::Right => {
                    self.point_x += 1;
                    self.reload_map = true;
                }
                VirtualKeyCode::Down => {
                    self.point_y += 1;
                    self.reload_map = true;
                }
                VirtualKeyCode::Escape => {
                    ctx.quit_requested = true;
                }
                _ => {

                }
            }
        }

        match input.keycode {
            None => {}
            Some(_) => {}
        }

        Ok(())
    }
}