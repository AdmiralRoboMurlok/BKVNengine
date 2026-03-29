use std::fs;
use std::path::Path;

mod gui;
mod json_handler;
mod tests;

fn main() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let _data_folder = exe_dir.join("data");

    let target_dir = Path::new("./data/characters");
    fs::create_dir_all(target_dir).unwrap();
    let target_dir = Path::new("./data/background");
    fs::create_dir_all(target_dir).unwrap();
    let target_dir = Path::new("./data/sound");
    fs::create_dir_all(target_dir).unwrap();
    let target_dir = Path::new("./data/project");
    fs::create_dir_all(target_dir).unwrap();

    let _ = gui::initialize_gui();
}

pub fn placeholder(test: &usize){
    println!("Hello, from placeholder, {}", test);
}