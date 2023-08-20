use std::{env, path};
use ggez::{event, GameResult};
use ggez::conf::{WindowMode, WindowSetup};

mod state;
mod file;
mod view;
mod asset;

fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("map-distribution", "icmir2")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("map-distribution"))
        .window_mode(WindowMode::default().dimensions(1400.0, 1200.0));

    let (mut ctx, event_loop) = cb.build()?;

    let mut app = view::App::new(&mut ctx);
    // state.load_tile(&mut ctx, 0);
    event::run(ctx, event_loop, app)
}
