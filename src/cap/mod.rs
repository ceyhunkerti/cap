use std::fs::{metadata, File};
use std::io::prelude::*;
use std::path::Path;

use crate::cap::assets::Assets;

pub mod assets;
pub mod html;

pub fn init(path: Option<String>) {
    let p = path.unwrap_or("cap.yml".to_string());

    let config_file = match metadata(&p) {
        Ok(md) => {
            if md.is_dir() {
                Path::new(&p).with_file_name("cap.yaml")
            } else {
                Path::new(&p).to_path_buf()
            }
        }
        _ => Path::new(&p).to_path_buf(),
    };
    let f = Assets::get(format!("defaults/config.yml").as_str()).unwrap();
    let content = std::str::from_utf8(f.data.as_ref()).unwrap().to_owned();

    let mut file = File::create(config_file).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.flush().unwrap();
}
