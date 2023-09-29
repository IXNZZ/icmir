use crate::map::MapAsset;

mod data;
mod config;
mod map;
mod cache;
mod frame;

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() {
    // let x = include_bytes!("/Users/vinter/Dev/Mir2/data/Hum.wzl");

    // data::convert_data();
    // map::check_map();
    // map::test_map();
    // let mut map = MapAsset::new("/Users/vinter/Dev/Mir2");
    // map.save("EM001.map", Arc::new(Semaphore::new(1))).await;
    // MapAsset::save_all().await;
    // frame::test_frame();
    let mut image_asset = cache::create_default_image_asset("/Users/vinter/Dev/Mir2");

    let option = image_asset.load_image(FileDesc::ZONE { file: 3, number: 0, index: 1 }, FileDescType::IDX);
    // let option = image_asset.load_image(FileDesc::ZONE { file: 3, number: 0, index: 1 }, FileDescType::IDX);
    let option = image_asset.load_image(FileDesc::ZONE { file: 3, number: 0, index: 0 }, FileDescType::IDX);
    let option = image_asset.load_image(FileDesc::ZONE { file: 3, number: 0, index: 1 }, FileDescType::IDX);
    if let Some(v) = option {
        println!("{},{},{},{}", v.height, v.width, v.offset_x, v.offset_y);
    }

    let map = cache::create_map("/Users/vinter/Dev/Mir2", "n0");
    if let Some(map) = map {
        println!("map: {}, {}, name: {}, len: {}", map.width, map.height, map.name, map.tiles.len());
    }
}


use chrono::Local;
use std::sync::Arc;
use tokio::{ self, runtime::Runtime, sync::Semaphore, time::{self, Duration}};
use crate::cache::{FileDesc, FileDescType};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

// fn main() {
//     let rt = Runtime::new().unwrap();
//     rt.block_on(async {
//         // 只有3个信号灯的信号量
//         let semaphore = Arc::new(Semaphore::new(3));
//
//         // 5个并发任务，每个任务执行前都先获取信号灯
//         // 因此，同一时刻最多只有3个任务进行并发
//         for i in 1..=5 {
//             let semaphore = semaphore.clone();
//                 let _permit = semaphore.acquire_owned().await.unwrap();
//             println!("i: {}", i);
//             tokio::spawn(async move {
//                 println!("{}, {}", i, now());
//                 time::sleep(Duration::from_secs(2)).await;
//                 drop(_permit);
//             });
//         }
//
//         time::sleep(Duration::from_secs(3)).await;
//     });
// }
