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
    name: String,
    last_change: String,
    characters: Vec<String>,
    backgrounds: Vec<String>,
    sounds: Vec<String>,
    scenes: Vec<Scene>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub first_launch: bool,
}