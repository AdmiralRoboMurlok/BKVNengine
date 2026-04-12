use std::fmt::format;
use std::time::SystemTime;
use serde_json::from_str;
use crate::json_handler::json_struct::{ProjectJSON, Scene};

pub fn create_scene(scene_id: u32, number_of_characters_in_scene: u8,
                    characters: Vec<String>, background: Vec<String>,
                    sound: Vec<String>) -> serde_json::error::Result<String> {
    let scene: &str = &format!(r#"
    {{
        id: {:?},
        number_of_characters: {:?},
        characters_present: {:?},
        background: {:?},
        sound: {:?},
    }}
    "#,
    scene_id, number_of_characters_in_scene, characters, background, sound);

    let scene_json = serde_json::to_string_pretty(scene);
    scene_json
}

pub fn create_project(name: &str, last_change: SystemTime, characters: Vec<String>,
                  backgrounds: Vec<String>, sounds: Vec<String>,
                  scene: serde_json::error::Result<String>) -> serde_json::error::Result<String> {
    let project: &str = &format!(r#"
    {{
        name: {:?},
        last_change: {:?},
        characters: {:?},
        backgrounds: {:?},
        sounds: {:?},
        scenes: {:?},
    }}
    "#,
    name, last_change, characters, backgrounds, sounds, scene);

    let project_json =  serde_json::to_string_pretty(project);
    project_json
}

pub fn create_config() -> serde_json::error::Result<String> {
    let config: &str = &format!(r#"
    {{
        first_launch: true
    }}"#);

    let config_file = serde_json::to_string_pretty(config);
    config_file
}