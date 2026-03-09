use rfd::FileDialog;
use std::fs;
use std::path::Path;

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

}

pub fn import_sound() {

}