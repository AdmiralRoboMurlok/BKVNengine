use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::json_handler::json_parser::{create_scene, create_project};
use crate::json_handler::json_struct::{Config, ProjectJSON, Scene};
use std::time::SystemTime;

pub mod json_reader;
mod json_parser;
pub mod json_struct;

pub fn create_first_project_json() -> Result<(), Box<dyn std::error::Error>> {
    let date_now = SystemTime::now();

    let project_data = create_project("FirstProject", date_now, Vec::new(), Vec::new(), Vec::new(), Vec::new());

    let target_dir = Path::new("./Projects/FirstProject/project");
    fs::create_dir_all(target_dir).unwrap();

    let mut json_file = File::create("./Projects/FirstProject/project/ProjectData.json")?;
    serde_json::to_writer_pretty(&mut json_file, &project_data).unwrap();

    let target_dir = Path::new("./Projects/FirstProject/data/characters");
    fs::create_dir_all(target_dir).unwrap();
    let target_dir = Path::new("./Projects/FirstProject/data/background");
    fs::create_dir_all(target_dir).unwrap();
    let target_dir = Path::new("./Projects/FirstProject/data/sound");
    fs::create_dir_all(target_dir).unwrap();
    
    Ok(())
}

pub fn create_project_json(project_name: &str) {
    let date_now = SystemTime::now();

    let project_data = create_project(project_name, date_now, Vec::new(), Vec::new(), Vec::new(), Vec::new());

    let mut json_file = File::create("./project/ProjectData.json");
    // json_file.unwrap().write_all(project_data.unwrap().as_bytes());
}

pub fn create_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_file = File::create("./config.json")?;

    let config = Config {
        first_launch: true,
    };

    serde_json::to_writer_pretty(&mut config_file, &config)?;
    Ok(())
}