use std::{env, path};
use ggez::{event, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

mod state;
mod file;
mod view;
mod asset;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%F %T%.6f"))
    }
}

fn main() -> GameResult {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "error,file=info,map_dist=debug")
    }
    tracing_subscriber::fmt::fmt()
        .with_timer(LocalTimer)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

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
