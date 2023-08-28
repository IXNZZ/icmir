use crate::map::MapAsset;

mod data;
mod config;
mod map;

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() {
    // let x = include_bytes!("/Users/vinter/Dev/Mir2/data/Hum.wzl");

    // data::convert_data();
    // map::check_map();
    // map::test_map();
    // let mut map = MapAsset::new(config::BASE_DIR);
    // map.save("n4.map").await;
    MapAsset::save_all().await;
}


use chrono::Local;
use std::sync::Arc;
use tokio::{ self, runtime::Runtime, sync::Semaphore, time::{self, Duration}};

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
