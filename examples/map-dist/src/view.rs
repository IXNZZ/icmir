use ggez::event::EventHandler;
use ggez::{Context, GameResult};

pub struct App {

}

impl App {

    pub fn new(ctx: &mut Context) -> Self {
        ctx.gfx.set_resizable(false).unwrap();
        ctx.gfx.set_drawable_size(1920.0, 1080.0).unwrap();
        let size = ctx.gfx.window().outer_size();
        println!("size: {}, {}", size.width, size.height);
        Self {}
    }
}

impl EventHandler<ggez::GameError> for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}