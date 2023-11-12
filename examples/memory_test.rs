use std::io::{Read, stdin};
use ggez::glam::u64vec2;
use moka::sync::Cache;

pub fn main() {
    // let mut std_str = String::new();
    //
    // let mut  stdin1 = stdin();
    // stdin1.read_line(&mut std_str).unwrap();
    // println!("str1: {}", std_str);
    // let mut std_vec:Vec<u8> = Vec::new();
    // stdin1.read_to_end(&mut std_vec).unwrap();
    // println!("str2: {:?}", std_vec);
    for image in 0..2 {
        test_in();
    }

}

fn test_in() {
    let mut cache:Cache<u64, Box<[u64; 102400]>> = moka::sync::CacheBuilder::default().build();
    // let mut data: Vec<Vec<u64>> = Vec::new();

    let mut std_str = String::new();
    let mut stdin = stdin();
    let mut key = 0;
    loop {
        std_str.clear();
        stdin.read_line(&mut std_str).unwrap();
        let c = std_str.trim();
        if c.eq("q")  { return; }
        if c.eq("a") {
            println!("command A:{}, len: {}", c, cache.iter().count());
            // let d = vec![1;102400];
            // cache.push(d);
            key += 1;
            let x = Box::new([1_u64; 102400]);
            cache.insert(key, x);
        }
        if c.eq("d") {
            println!("command D:{}, len: {}", c, cache.iter().count());
            if cache.iter().count() > 1 {
                // data.remove(data.len() - 1);
            }
        }
        if c.eq("r") {
            println!("command R:{}, len: {}", c, cache.iter().count());
            cache.invalidate_all();

        }

    }
}