pub mod file_reader;

use rfd::FileDialog;
use std::fs;
use std::path::Path;
use crate::gui::toolbar_enums::{EditOptions, FilesOptions, FilterOptions, HelpOptions, ViewOptions};

pub fn import_character() {
    if let Some(path) = FileDialog::new().add_filter("characters", &["png", "jpg", "jpeg"]).set_directory("/").pick_file() {
        let file_name = path.file_name().unwrap();

        let target_dir = Path::new("./data/characters");
        fs::create_dir_all(target_dir).unwrap();

        let target_path = target_dir.join(file_name);

        fs::copy(&path, &target_path).unwrap();
    }
}

pub fn import_background() {
    if let Some(path) = FileDialog::new().add_filter("background", &["png", "jpg", "jpeg"]).set_directory("/").pick_file() {
        let file_name = path.file_name().unwrap();

        let target_dir = Path::new("./data/background");
        fs::create_dir_all(target_dir).unwrap();

        let target_path = target_dir.join(file_name);

        fs::copy(&path, &target_path).unwrap();
    }
}

pub fn import_sound() {
    if let Some(path) = FileDialog::new().add_filter("sound", &["mp3", "wav", "aac"]).set_directory("/").pick_file() {
        let file_name = path.file_name().unwrap();

        let target_dir = Path::new("./data/sound");
        fs::create_dir_all(target_dir).unwrap();

        let target_path = target_dir.join(file_name);

        fs::copy(&path, &target_path).unwrap();
    }
}

pub fn filter_apply(option: &FilterOptions) {
    
}

pub fn toolbar_file_handler(option: &FilesOptions) {
    
}

pub fn toolbar_edit_handler(option: &EditOptions) {

}

pub fn toolbar_view_handler(option: &ViewOptions) {

}

pub fn toolbar_help_handler(option: &HelpOptions) {

}