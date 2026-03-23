use std::fs;

//fn load_all_files() -> Vec<String> {

//}

fn load_all_choice(path: String) -> Vec<String> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            entry.ok().map(|e| {
                e.file_name().to_string_lossy().to_string()
            })
        })
        .collect()
}