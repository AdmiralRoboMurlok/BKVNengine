use std::fs;

//fn load_all_files() -> Vec<String> {

//}

pub fn load_all_choice(path: &str) -> Vec<String> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            entry.ok().map(|e| {
                e.file_name().to_string_lossy().to_string()
            })
        })
        .collect()
}