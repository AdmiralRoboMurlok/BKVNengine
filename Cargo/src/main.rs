mod gui;
mod json_handler;

fn main() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let data_folder = exe_dir.join("data");

    gui::initialize_gui();
}

pub fn placeholder(){
    println!("Hello, from placeholder");
}