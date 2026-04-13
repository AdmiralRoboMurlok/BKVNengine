use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::json_handler::json_parser::{create_scene, create_project};
use crate::json_handler::json_struct::{Config, ProjectJSON, Scene};
use std::time::SystemTime;

pub mod json_reader;
mod json_parser;
pub mod json_struct;

pub fn create_first_project_json() {
    let scene: serde_json::error::Result<String> = create_scene(0, 1, Vec::new(), Vec::new(), Vec::new());

    let date_now = SystemTime::now();

    let project_data = create_project("FirstProject", date_now, Vec::new(), Vec::new(), Vec::new(), scene);

    let mut json_file = File::create("./project/ProjectData.json");
    json_file.unwrap().write_all(project_data.unwrap().as_bytes());
}

pub fn create_project_json(project_name: &str) {
    let scene: serde_json::error::Result<String> = create_scene(0, 1, Vec::new(), Vec::new(), Vec::new());

    let date_now = SystemTime::now();

    let project_data = create_project(project_name, date_now, Vec::new(), Vec::new(), Vec::new(), scene);

    let mut json_file = File::create("./project/ProjectData.json");
    json_file.unwrap().write_all(project_data.unwrap().as_bytes());
}

pub fn create_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_file = File::create("./config.json")?;

    let config = Config {
        first_launch: true,
    };

    serde_json::to_writer_pretty(&mut config_file, &config)?;
    Ok(())
}