use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::de::Error;
use crate::json_handler::json_struct::Config;

pub fn read_project() {

}

pub fn read_config() -> Result<Config, std::io::Error> {
    let config_path = Path::new("./config.json");
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);

    let options:Config = serde_json::from_reader(reader)?;

    Ok(options)
}