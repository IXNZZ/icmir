use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;
use bytes::{Buf};
use crate::config;


pub fn convert_data() {

    let dir = Path::new(config::BASE_DIR).join(config::DATA_DIR_NAME).read_dir().unwrap();
    let mut files:Vec<String> = dir.map(|x| {
        String::from(x.unwrap().path().to_str().unwrap())
    }).filter(|x| { x.ends_with(".wzx") }).collect();
    files.sort();
    // let output_dir = "/Users/vt/Documents/LegendOfMir/data";
    let mut sum = 0;
    for i in files {
        let (name, _) = i.split_at(i.len() - 4);
        let wzl = String::from(name) + ".wzl";
        let index = check_file(i.as_str(), wzl.as_str());
        let path = Path::new(config::BASE_DIR).join(config::DATA_DIR_NAME).join(name.to_string() + ".idx");
        if path.exists() {
            fs::remove_file(path.clone()).unwrap();
        }
        sum += index.len();
        let mut idx = File::create(path).unwrap();
        for i in index {
            idx.write_all(&u32::to_le_bytes(i)[..]).unwrap();
        }
        idx.flush().unwrap();
        // println!("index count:{}, file: {}", index.len(), name);
    }
    println!("success: {}", sum);
}

pub fn check_file(wzx: &str, wzl: &str) -> Vec<u32> {
    let idx_data = read_wzx(wzx);
    let index = read_wzl(wzl, idx_data.as_slice());
    // for idx in idx_data {
    //     if !index.contains(&idx) {
    //         println!("file: {}, idx: {}", wzl, idx);
    //     }
    // }
    index
}


fn read_wzl(path: &str, idx: &[u32]) -> Vec<u32> {
    let path = Path::new(path);
    let file_name = path.file_name().unwrap();
    let mut file = File::open(path).unwrap();
    let file_size = file.metadata().unwrap().len();
    let mut header = [0; 48];
    file.read(&mut header).unwrap();
    let mut header = &header[44..];
    let length = header.get_u32_le();
    let mut index:Vec<u32> = Vec::new();

    if file_size <= 64 {
        // index.push(48);
        return index;
    }

    let mut body = [0; 16];
    let mut count = 0;
    let mut reader = BufReader::new(file);
    let mut pos = 48;
    let mut prev = 48;
    let mut prev_len = 0;
    'runner: loop {
        pos = reader.stream_position().unwrap();
        if pos >= file_size {
            break;
        }
        read_data(&mut reader, &mut body);
        let wzl = new_wzl(&mut body);
        if !(((wzl.pixel == 3 || wzl.pixel == 5) && (wzl.compressed == 1 || wzl.compressed == 9 || wzl.compressed == 5) && wzl.width < 20000) ||
            (wzl.pixel == 0 && wzl.compressed == 0 && wzl.length == 0 && wzl.height == 0 && wzl.width == 0)) {

            reader.seek(SeekFrom::Start(prev)).unwrap();
            read_data(&mut reader, &mut body);
            let wzl = new_wzl(&mut body);
            if (wzl.width == 1 || wzl.width == 4) && wzl.height == 1 && wzl.length == 0 {
                if wzl.pixel == 5 {
                    reader.seek(SeekFrom::Current(8)).unwrap();
                } else {
                    reader.seek(SeekFrom::Current(4)).unwrap();
                }
                continue 'runner;
            }
            for idx in &idx[..] {
                if *idx > prev as u32 {
                    reader.seek(SeekFrom::Start(*idx as u64)).unwrap();
                    prev_len = *idx as u64 - prev - 16;
                    continue 'runner;
                }
            }

            break;
        }

        if prev_len != 0 && prev_len + 16 + prev != pos {
            println!("idx: {:05}, prev: {}, prev_len: {}, pos: {:08}, wzl: {:?}, file: {:?}", count, prev, prev_len, pos, wzl, file_name);
        }

        index.push(pos as u32);
        prev_len = wzl.length as u64;
        prev = pos;
        count += 1;
        if wzl.length == 0 {
            if wzl.width != 0 && wzl.height != 0 {
                if (wzl.width == 1 || wzl.width == 4) && wzl.height==1 {
                    if wzl.pixel == 5 {
                        reader.seek(SeekFrom::Current(4)).unwrap();
                    } else {
                        reader.seek(SeekFrom::Current(16)).unwrap();
                    }
                    continue;
                }

                for idx in &idx[..] {
                    if *idx > pos as u32 {
                        let sub = *idx - pos as u32 - 16;
                        if sub > 432 {
                            let seek = ((wzl.width + 1) / 2 * 2) * wzl.height * 2;

                            reader.seek(SeekFrom::Current(seek as i64)).unwrap();
                            break;
                        }
                        reader.seek(SeekFrom::Start(*idx as u64)).unwrap();
                        break;
                    }
                }
            }
        } else {
            reader.seek(SeekFrom::Current(wzl.length as i64)).unwrap();
        }
    }
    index.push(file_size as u32);
    if pos != file_size {
        let sub = if count > length { count - length} else { length - count };
        println!("total ==---> imageCount: {:05}, length: {:05}, checkCount: {:03}, checkSize: {}, prev: {:05}, pos: {:08}, fileSize: {:08}, file: {:?}",
                 count, length, sub, pos == file_size, prev, pos, file_size, file_name);
    }
    index
}

pub fn read_wzx(path: &str) -> Vec<u32> {
    let path = Path::new(path);
    let _file_name = path.file_name().unwrap();
    let mut file = File::open(path).unwrap();
    let len = file.metadata().unwrap().len();
    let mut buffer = vec![0; len as usize];
    file.read(&mut buffer[..]).unwrap();
    let mut buffer = &buffer[44..];
    let _count = buffer.get_u32_le();

    let num = (len as usize - 48) / 4;
    let mut data = Vec::with_capacity(num);
    for _i in 0..num {
        let value = buffer.get_u32_le();
        if value == 0 { continue }
        data.push(value);
    }
    data.sort();
    //println!("wzx======>count: {}, fileSize: {}, num: {}, num==count: {}, dataSize(NotEmpty): {}, fileName: {:?}", count, len, num, count == num as u32, data.len(), file_name);

    data
}

fn new_wzl(body: &[u8]) -> Wzl {
    let mut body = body;
    let pixel = body.get_u8();
    let compressed = body.get_u8();
    let reserve = body.get_u8();
    let compress_level = body.get_u8();

    let width = body.get_u16_le();
    let height = body.get_u16_le();
    let offset_x = body.get_i16_le();
    let offset_y = body.get_i16_le();
    let length = body.get_u32_le();
    Wzl {pixel, compressed, reserve, compress_level, width, height, offset_x, offset_y, length}
}

pub fn read_data(reader: &mut BufReader<File>, buffer: &mut [u8]) {
    let length = buffer.len();
    let len = reader.read(buffer).unwrap();
    if len < length {
        reader.read(&mut buffer[len..]).unwrap();
    }
}

#[derive(Debug, Default)]
pub struct Wzl {
    pub pixel: u8,
    pub compressed: u8,
    pub reserve: u8,
    pub compress_level: u8,
    pub width: u16,
    pub height: u16,
    pub offset_x: i16,
    pub offset_y: i16,
    pub length: u32
}