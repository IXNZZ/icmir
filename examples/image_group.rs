use ggez::conf::{WindowMode, WindowSetup};
use ggez::GameResult;
use ggez::graphics::{Image, ImageFormat, ScreenImage};

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("map-distribution", "icmir2")
        // .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("map-distribution"))
        .window_mode(WindowMode::default().dimensions(1400.0, 1200.0));

    let (mut ctx, event_loop) = cb.build()?;

    // let mut  screen = ScreenImage::new(&ctx, None, 1.0, 1.0, 1);
    // let image = screen.image(&ctx);
    let mut image = Image::new_canvas_image(&ctx, ImageFormat::Rgba8UnormSrgb, 1500, 2000, 1);

    println!("image: {}:{}", image.width(), image.height());


    // let mut app = App::new(&mut ctx);
    // // state.load_tile(&mut ctx, 0);
    // my_event::run(ctx, event_loop, app)

    // ctx.gfx.

    Ok(())
}