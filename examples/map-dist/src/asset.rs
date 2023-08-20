use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use file::data::ImageData;

pub struct MapAsset {
    pub map_file_name: String,
    pub x_point: u32,
    pub y_point: u32,
    pub back_image: Vec<ImageData>
}

pub struct ImageAsset {
    dir: String,
    pub image: HashMap<String, ImageData>,
    pub index: HashMap<String, Vec<u32>>,
}

impl ImageAsset {

    pub fn new(dir: &str) -> Self {
        Self {dir: dir.to_string(), image: HashMap::new(), index: HashMap::new()}
    }
    pub fn load_image_asset(&mut self, name: &str, file: u8, idx: u32) -> Option<&ImageData> {
        let key = convert_file_name(self.dir.as_str(), name, file);
        if !self.image.contains_key(&key) {
            if !self.index.contains_key(&key) {
                let index = file::data::load_index(key.as_str());
                self.index.insert(key.clone(), index);
            }
            if let Some(index) = self.index.get(key.as_str()) {
                let i = index[idx as usize];
                let image_data = file::data::load_image(key.as_str(), i, i + 16);
                self.image.insert(key.clone(), image_data);
            }
        }

        self.image.get(&key)
    }
}


fn convert_file_name(dir: &str, name: &str, file: u8) -> String {
    if file == 0 {
        format!("{}{}.wzx", dir, name)
    } else {
        format!("{}{}{}.wzx", dir, name, file)
    }
}

