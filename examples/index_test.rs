use ggez::conf::{WindowMode, WindowSetup};
use ggez::{Context, event, GameError, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::glam::vec2;
use ggez::graphics::{BlendMode, Canvas, Color, Drawable, DrawMode, DrawParam, FillOptions, InstanceArray, Mesh, Rect, ZIndex};
use winit::event::MouseButton;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("hum-distribution", "icmir2")
        // .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("hum-distribution"))
        .window_mode(WindowMode::default().dimensions(1400.0, 1200.0));

    let (mut ctx, event_loop) = cb.build()?;

    let app = App::new(&mut ctx);
    event::run(ctx, event_loop, app);
}

pub struct App {
    x: f32,
    y: f32,
    distance: f32,
    angle: f32,
    sharing: f32,
}

impl App {

    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            x: 500.0,
            y: 500.0,
            distance: 0.,
            angle: 0.,
            sharing: 0.,
        }
    }

    fn angle2(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
        // 180.0 / std::f32::consts::PI = 57.295776
        (dst_y - src_y).atan2(dst_x - src_x) * 57.295776

        // if angle < 0. { angle + 360.} else { angle }
    }

    fn sharing2(angle: f32, sharing: u32) -> f32 {
        // let sub = 90.0 + 360.0 / 2.0 / sharing as f32 + angle;
        // let angle = if sub < 0.0 { sub + 360.0 } else { sub };
        // let sub = 360.0 / sharing as f32;
        // for s in 0..sharing {
        //     if angle >= s as f32 * sub && angle < (s as f32 + 1.0) * sub {
        //         return s as f32 + 1.0;
        //     }
        // }
        // return sub;
        let angle = if angle + SHARING_8 < 0. { angle + SHARING_8 + 360.0 } else { angle + SHARING_8 };
        println!("calc angle: {}", angle);
        // println!("an: {}", 180.0 / std::f32::consts::PI);
        ((angle) / (360. / 8.0) + 1.0).floor()
    }

    pub fn distance2(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
        ((dst_x - src_x).abs().powi(2) + (dst_y - src_y).abs().powi(2)).sqrt()
    }
}

const SHARING_8: f32 = 90.0 + 360.0 / 2.0 / 8.0;
const SHARING_12: f32 = 90.0 + 360.0 / 2.0 / 12.0;
const SHARING_16: f32 = 90.0 + 360.0 / 2.0 / 16.0;

impl EventHandler<GameError> for App {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::new(0.1, 0.2, 0.3, 1.0));
        // let y_line = ggez::graphics::Mesh::new_line(ctx, &[vec2(500.0, 0.0), vec2(500.0, 1000.0)], 1.0, Color::RED).unwrap();
        // let x_line = ggez::graphics::Mesh::new_line(ctx, &[vec2(0.0, 500.0), vec2(1000.0, 500.0)], 1.0, Color::RED).unwrap();
        // canvas.draw(&x_line, DrawParam::default());
        // canvas.draw(&y_line, DrawParam::default());
        // let mut text = ggez::graphics::Text::new(format!("point: {}:{}", self.x, self.y));
        // text.set_scale(32.0);
        // canvas.draw(&text,DrawParam::new().dest(vec2(100., 1000.)));
        //
        // let line = graphics::Mesh::new_line(ctx, &[vec2(500., 500.), vec2(self.x, self.y)], 1.0, Color::GREEN)?;
        //
        // canvas.draw(&line, DrawParam::default());
        //
        // let mut text = ggez::graphics::Text::new(format!("distance: {} \n angle: {} \n sharing: {}",
        //                                                  self.distance, self.angle, self.sharing));
        // text.set_scale(32.0);
        // canvas.draw(&text,DrawParam::new().dest(vec2(100., 1050.)));

        let red_rect = Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), Rect::new_i32(50, 50, 500, 500), Color::RED).unwrap();
        let green_rect = Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), Rect::new_i32(200, 200, 500, 500), Color::GREEN).unwrap();
        canvas.draw(&red_rect, DrawParam::default().z(ZIndex::from(12)));
        canvas.set_blend_mode(BlendMode::ADD);
        canvas.draw(&green_rect, DrawParam::default().z(ZIndex::from(11)));
        // InstanceArray::new()
        canvas.finish(ctx)?;
        Ok(())
    }


}

