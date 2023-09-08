use std::{fs, io};
use std::collections::HashMap;
use std::iter::Map;
use std::path::{Path, PathBuf};
use bytes::{Buf, Bytes};
use moka::sync::{Cache, CacheBuilder};

pub fn test_cache() {
    // let cache: Cache<u64, Bytes> = CacheBuilder::new(10000).build();
    test_dir_main();
}

fn test_dir_main() -> io::Result<()> {
    let mut entries = fs::read_dir("/Users/vinter/Dev/Mir2/data/")?
        .map(|res| res.map(|e| e.path()).unwrap()).filter(|x| {
        println!("{:?}, idx: {:?}", x, x.to_str().unwrap().ends_with(".idx"));
        x.to_str().unwrap().ends_with(".idx")
    }).collect::<Vec<PathBuf>>();

    // 不保证 `read_dir` 返回条目的顺序。
    // 如果需要可重复的排序，则应对条目进行显式排序。

    entries.sort();
    let mut sum = 0;
    let count =  entries.len();
    for entry in entries {
        let size = entry.metadata().unwrap().len();
        println!("{:?}, size: {}", entry, size);
        sum += size;
    }

    println!("sum: {sum},count: {}", count);

    // 现在，条目已经按照路径进行了排序。


    Ok(())
}

const IMAGE_DIR: &str = "data";
const IMAGE_FILE_SUFFIX: &str = "wzl";
const IMAGE_IDX_SUFFIX: &str = "idx";
const IMAGE_WZX_SUFFIX: &str = "wzx";

#[derive(Clone)]
pub struct WzlImage {
    pub width: u16,
    pub height: u16,
    pub offset_x: i16,
    pub offset_y: i16,
    pub bytes: Bytes,
}

/// 数据KEY格式
/// 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000
///     |8位文件类型|8位序号   | 12位文件名称   | 32位数据文件索引
/// 文件类型 0: wzl
///         1: wzx
///         2: idx
///         3: map
///         4: wav
pub enum ImageDesc {
    KEY (u64),
    KEYS (Vec<u64>),
    ZONE {file: u16, number: u16, index: u32 },
    ZONES {file: u16, number: u16, index: Vec<u32> },
    ORDER {file: u16, number: u16, index: u32, count: u32 }

}

impl ImageDesc {

    pub fn get_file_key(&self) -> u32 {

    }

}

pub struct ImageAsset {
    dir: String,
    file_map: HashMap<u32, String>,
    index_map: HashMap<u32, Vec<u32>>,
    wzx_map: HashMap<u32, Vec<u32>>,
    image_cache: Cache<u64, WzlImage>
}

impl ImageAsset {

    pub fn new(dir: String) -> Self {

        ImageAsset {
            dir, file_map: HashMap::with_capacity(1024),
            index_map: HashMap::with_capacity(1024),
            wzx_map: HashMap::with_capacity(1024),
            image_cache: Cache::new(10_000)
        }
    }

    pub fn put_file_map(&mut self, key: u32, value: &str, preload_index: bool) {
        self.file_map.insert(key, value.to_string());
        if preload_index {

            let collect = |buf: &PathBuf, skip: usize| {
                if let Ok(p) = fs::read(buf) {
                    p.chunks(4).skip(skip).map(|x| {
                        let t: [u8; 4] = [x[0], x[1], x[2], x[3]];
                        u32::from_le_bytes(t)
                    }).collect()
                } else {
                    Vec::new()
                }
            };

            let path = Path::new(self.dir.as_str()).join(IMAGE_DIR).join(value);
            // let path = path.join(value);
            println!("{:?}", path);
            let buf = path.with_extension(IMAGE_IDX_SUFFIX);
            // println!("{:?}", buf);
            self.index_map.insert(key, collect(&buf, 0));
            let buf = path.with_extension(IMAGE_WZX_SUFFIX);
            // println!("{:?}", buf);
            self.wzx_map.insert(key, collect(&buf, 12));
            // println!("test: idx:{}, wzx: {}", self.index_map.len(), )
        }
    }

    pub fn read_idx(&mut self, key: ImageDesc) {

    }


    pub fn read_wzx(&mut self, key: ImageDesc) {

    }

}


pub fn create_default_image_asset(dir: &str) -> ImageAsset {
    let mut asset = ImageAsset::new(dir.to_string());
    asset.put_file_map(0, "tiles", true);
    asset.put_file_map(1, "smTiles", true);
    asset.put_file_map(2, "objects", true);

    asset
}

