use crate::json_handler::json_struct;
use serde_json::from_str;
use crate::json_handler::json_struct::{ProjectJSON, Scene};

pub fn create_scene(scene_id: u32, number_of_characters_in_scene: u8,
                characters: Vec<String>, background: Vec<String>,
                sound: Vec<String>) {
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

    let scene_json = from_str::<Scene>(scene);
}

pub fn create_project(name: String, last_change: String, characters: Vec<String>,
                  backgrounds: Vec<String>, sounds: Vec<String>,
                  scene: Scene) {
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

    let project_json = from_str::<ProjectJSON>(project);
}