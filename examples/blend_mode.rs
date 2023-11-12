use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::{Context, event, GameError, GameResult};
use ggez::glam::vec2;
use ggez::graphics::{BlendFactor, BlendMode, BlendOperation, Canvas, Color, Drawable, DrawParam, Image, InstanceArray, PxScale, ScreenImage, Text};
use ggez::input::keyboard::KeyInput;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use winit::event::VirtualKeyCode;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%F %T%.6f"))
    }
}

pub fn main() -> GameResult {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "warn,blend_mode=debug")
    }
    tracing_subscriber::fmt::fmt()
        .with_timer(LocalTimer)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cb = ggez::ContextBuilder::new("hum-distribution", "icmir2")
        .resources_dir_name("/Users/vt/Documents/LegendOfMir/data/save")
        .window_setup(WindowSetup::default().title("hum-distribution"))
        .window_mode(WindowMode::default().dimensions(2000.0, 1200.0));

    let (mut ctx, event_loop) = cb.build()?;

    let mut app = App::new(&mut ctx);
    // state.load_tile(&mut ctx, 0);
    event::run(ctx, event_loop, app)
}

pub struct App {
    bg: Image,
    object: Image,
    object1: Image,
    object2: Image,
    object3: Image,
    array: InstanceArray,

    select: u8,
    src_color: i32,
    dst_color: i32,
    color_mode: i32,
    src_alpha: i32,
    dst_alpha: i32,
    alpha_mode: i32,

    blend: BlendMode,
}

impl App {

    pub fn new(_ctx: &mut Context) -> Self {
        let bg = ScreenImage::new(_ctx, None, 1.0, 1.0, 1).image(_ctx);
        let object = ScreenImage::new(_ctx, None, 1.0, 1.0, 1).image(_ctx);
        let draw: Vec<DrawParam> = (0..200).map(|i|{
            let x = (i % 20) * 96;
            let y = (i / 20) * 64;
            DrawParam::default().dest(vec2(x as f32, y as f32)).scale(vec2(2.0, 2.0)).z(x)
        }).collect();
        let image = Image::from_path(_ctx, "/smtiles_00001.png").unwrap();
        let mut array = InstanceArray::new(_ctx, image);
        array.set(draw);

        let object1 = Image::from_path(_ctx, "/humeffect_00001.png").unwrap();
        let object2 = Image::from_path(_ctx, "/magic_00035.png").unwrap();
        let object3 = Image::from_path(_ctx, "/cbohum7_00001.png").unwrap();

        // println!("x: {:?}", BlendFactor::Dst);
        // BlendFactor::from(1);
        Self {bg,
            object,
            object1,
            object2,
            object3,
            array,
            blend: BlendMode::ADD,
            select: 0,
            src_color: 5,
            src_alpha: 10,
            dst_color: 2,
            dst_alpha: 2,
            color_mode: 1,
            alpha_mode: 1,
        }
    }

    fn select_factor(&self, factor: i32) -> BlendFactor {

        match factor {
            1 => { BlendFactor::Zero }
            2 => { BlendFactor::One }
            3 => { BlendFactor::Src }
            4 => { BlendFactor::OneMinusSrc }
            5 => { BlendFactor::SrcAlpha }
            6 => { BlendFactor::OneMinusSrcAlpha }
            7 => { BlendFactor::Dst }
            8 => { BlendFactor::OneMinusDst }
            9 => { BlendFactor::DstAlpha }
            10 => { BlendFactor::OneMinusDstAlpha }
            11 => { BlendFactor::SrcAlphaSaturated }
            12 => { BlendFactor::Constant }
            13 => { BlendFactor::OneMinusConstant }

            _ => {BlendFactor::Zero}
        }
    }

    fn select_mode(&self, mode: i32) -> BlendOperation {
        match mode {
            1 => {BlendOperation::Add}
            2 => {BlendOperation::Subtract}
            3 => {BlendOperation::ReverseSubtract}
            4 => {BlendOperation::Min}
            5 => {BlendOperation::Max}
            _ => {BlendOperation::Add}
        }
    }

    fn change(&mut self, inc: i32) {
        match self.select {
            1 => {
                self.src_color += inc;
                if self.src_color < 1 { self.src_color = 13 }
                if self.src_color > 13 { self.src_color = 1 }
                self.blend.color.src_factor = self.select_factor(self.src_color);
            },
            2 => {
                self.dst_color += inc;
                if self.dst_color < 1 { self.dst_color = 13 }
                if self.dst_color > 13 { self.dst_color = 1 }
                self.blend.color.dst_factor = self.select_factor(self.dst_color)
            },
            3 => {
                self.color_mode += inc;
                if self.color_mode < 1 { self.color_mode = 5 }
                if self.color_mode > 5 { self.color_mode = 1 }
                self.blend.color.operation = self.select_mode(self.color_mode)
            },
            4 => {
                self.src_alpha += inc;
                if self.src_alpha < 1 { self.src_alpha = 13 }
                if self.src_alpha > 13 { self.src_alpha = 1 }
                self.blend.alpha.src_factor = self.select_factor(self.src_alpha)
            },
            5 => {
                self.dst_alpha += inc;
                if self.dst_alpha < 1 { self.dst_alpha = 13 }
                if self.dst_alpha > 13 { self.dst_alpha = 1 }
                self.blend.alpha.dst_factor = self.select_factor(self.dst_alpha)
            },
            6 => {
                self.alpha_mode += inc;
                if self.alpha_mode < 1 { self.alpha_mode = 5 }
                if self.alpha_mode > 5 { self.alpha_mode = 1 }
                self.blend.alpha.operation = self.select_mode(self.alpha_mode)
            },
            _ => {}
        }
    }

    fn draw_text(&self, canvas: &mut Canvas, str: String, x: f32, y: f32, selected: bool) {
        let color = if selected { Color::RED } else { Color::GREEN };
        Text::new(str)
            .set_scale(PxScale::from(24.0))
            .draw(canvas, DrawParam::default().dest(vec2(x, y)).color(color));
    }


}

impl EventHandler<GameError> for App {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        // self.blend.color.dst_factor = BlendFactor::Dst;
        // if self.src_color > 0 {
        //
        // }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(_ctx, Color::new(0.1, 0.2, 0.3, 0.0));
        let mut bg = Canvas::from_image(_ctx, self.bg.clone(), Color::new(0.0, 0.0, 0.0, 0.0));
        let mut object = Canvas::from_image(_ctx, self.object.clone(), Color::new(0.0, 0.0, 0.0, 0.0));
        bg.draw(&self.array, DrawParam::default().dest(vec2(40.0, 200.0)));
        bg.finish(_ctx)?;
        object.set_blend_mode(self.blend);
        // object.draw()
        object.draw(&self.object1, DrawParam::default().scale(vec2(2.0, 2.0)).dest(vec2(10.0, 0.0)));
        object.draw(&self.object2, DrawParam::default().scale(vec2(2.0, 2.0)).dest(vec2(100.0, 0.0)));
        // object.set_blend_mode(BlendMode::ALPHA);
        object.draw(&self.object3, DrawParam::default().scale(vec2(2.0, 2.0)).dest(vec2(100.0, 550.0)));
        object.draw(&self.array, DrawParam::default().dest(vec2(0.0, 0.0)));
        // object.draw(&self.object3, DrawParam::default().scale(vec2(2.0, 2.0)).dest(vec2(100.0, 100.0)).z(30));

        object.finish(_ctx)?;

        canvas.draw(&self.object, DrawParam::default().dest(vec2(40.0, 300.0)));

        self.draw_text(&mut canvas, format!("SrcColor: {:?}", self.blend.color.src_factor), 50., 50., self.select == 1);
        self.draw_text(&mut canvas, format!("DstColor: {:?}", self.blend.color.dst_factor), 500., 50., self.select == 2);
        self.draw_text(&mut canvas, format!("ColorMode: {:?}", self.blend.color.operation), 1000., 50., self.select == 3);
        self.draw_text(&mut canvas, format!("srcAlpha: {:?}", self.blend.alpha.src_factor), 50., 100., self.select == 4);
        self.draw_text(&mut canvas, format!("DstAlpha: {:?}", self.blend.alpha.dst_factor), 500., 100., self.select == 5);
        self.draw_text(&mut canvas, format!("AlphaMode: {:?}", self.blend.alpha.operation), 1000., 100., self.select == 6);

        canvas.finish(_ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        match input.keycode {
            Some(VirtualKeyCode::Up) => {
                if self.select > 0 && self.select < 7 {
                    self.change(-1);
                    // info!("color: {}", self.src_color);

                }
            },
            Some(VirtualKeyCode::Down) => {
                if self.select > 0 && self.select < 7 {
                    self.change(1);
                }
            },
            Some(VirtualKeyCode::Left) => {
                if self.select <= 1 { self.select = 7 }
                self.select -= 1;
            },
            Some(VirtualKeyCode::Right) => {
                if self.select >= 6 { self.select = 0 }
                self.select += 1;
            },
            Some(VirtualKeyCode::Escape) => {
                ctx.quit_requested = true;
            }
            _ => {}
        }

        Ok(())
    }
}