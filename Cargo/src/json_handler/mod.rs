use crate::json_handler::json_parser::{create_scene, create_project};
use crate::json_handler::json_struct::{ProjectJSON, Scene};
use std::time::SystemTime;

mod json_reader;
mod json_parser;
pub mod json_struct;

pub fn create_first_project_json() {
    let scene: serde_json::Result<Scene> = create_scene(0, 1, Vec::new(), Vec::new(), Vec::new());

    let date_now = SystemTime::now();

    let project: serde_json::Result<ProjectJSON> = create_project("FirstProject", date_now, Vec::new(), Vec::new(), Vec::new(), scene);
}

pub fn create_project_json() {
    
}