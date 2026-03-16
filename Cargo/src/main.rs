mod gui;
mod json_handler;

fn main() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let _data_folder = exe_dir.join("data");

    let _ = gui::initialize_gui();
}

pub fn placeholder(test: &usize){
    println!("Hello, from placeholder, {}", test);
}