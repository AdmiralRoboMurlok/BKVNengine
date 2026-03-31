use crate::json_handler::json_struct;
use serde_json::from_str;

fn json_parser() {
    const SCENE: &str = r#"
    {

    }
    "#;

    const PROJECT: &str = r#"
    {
        name: "{}",
        last_change: "{}",
        characters: "{}",
        backgrounds: "{}",
        sounds: "{}",
        scenes: "{}",
    }
    "#;
}