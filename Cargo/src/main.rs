use std::fs;
use std::fs::File;
use std::path::Path;
use serde_json::json;
use crate::json_handler::{create_config_file, create_first_project_json};
use crate::json_handler::json_reader::read_config;

mod gui;
mod json_handler;
mod tests;

fn main() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let _data_folder = exe_dir.join("data");

    if (Path::new("./config.json").exists() == true) {
        let config_data = read_config();
        println!("{:#?}", config_data);
    }
    else {
        create_config_file();
        create_first_project_json();
    }
    
    /*
    let target_dir = Path::new("./data/characters");
    fs::create_dir_all(target_dir).unwrap();
    let target_dir = Path::new("./data/background");
    fs::create_dir_all(target_dir).unwrap();
    let target_dir = Path::new("./data/sound");
    fs::create_dir_all(target_dir).unwrap();
    let target_dir = Path::new("./project");
    fs::create_dir_all(target_dir).unwrap();
    */
    
    let _ = gui::initialize_gui();
}

pub fn placeholder(test: &usize){
    println!("Hello, from placeholder, {}", test);
}