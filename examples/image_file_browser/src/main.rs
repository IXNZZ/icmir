use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use iced::{Sandbox, Settings};
use lazy_static::lazy_static;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
// #[macro_use]
// extern crate lazy_static;

mod state;

lazy_static! {
 pub static ref FONT_BYTES: Vec<u8> = {
    let mut b = Vec::new();
    File::open("../../font/AlibabaPuHuiTi-2-55-Regular.otf").unwrap().read_to_end(&mut b).unwrap();
    b
    };
}

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%F %T%.6f"))
    }
}
 //include_bytes!("../../font/AlibabaPuHuiTi-2-55-Regular.woff")[..];

fn main() -> iced::Result {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "error,file=debug,image_file_browser=debug")
    }
    tracing_subscriber::fmt::fmt()
        .with_timer(LocalTimer)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    // let x = std::include_bytes!("../../font/AlibabaPuHuiTi-2-55-Regular.otf");
    // let mut b = Vec::new();
    // {
    //     let mut file = File::open("").unwrap();
    //     file.read_to_end(&mut b).unwrap();
    //     let bytes: &'static [u8] = b.as_slice();
    //     // let _ = font::load(Cow::from(bytes));
    // }
    // let _ = font::load(Cow::from(FONT_BYTES.as_slice()));

    state::State::run(Settings {
        default_font: Some(&FONT_BYTES),
        ..Settings::default()
    })
}

