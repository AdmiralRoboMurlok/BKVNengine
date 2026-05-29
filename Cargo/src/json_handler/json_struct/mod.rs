/*
{
    "project": [
        name: project_name,
        last_chage: date_of_last_change,
        characters: [paths to characters],
        backgrounds: [paths to backgrounds],
        sounds: [paths to sounds],
        scenes: [ // scene object
            {
            number_of_characters: defines their position later on,
            characters_present: [paths to characters],
            background: [path to background],
            sound: [path to sound],
            },
            // another scene obecjt,
        ]
        }
    ]
}
 */
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Scene {
    id: u32,
    number_of_characters: u8,
    characters_present: Vec<String>,
    background: Vec<String>,
    sound: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectJSON {
    pub name: String,
    pub last_change: SystemTime,
    pub characters: Vec<String>,
    pub backgrounds: Vec<String>,
    pub sounds: Vec<String>,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub first_launch: bool,
    pub current_project: String,
}