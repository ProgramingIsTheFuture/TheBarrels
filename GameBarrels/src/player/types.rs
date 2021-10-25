use bevy::prelude::{ColorMaterial, Handle};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub char_code: i8,
    pub username: String,
}

impl Default for Player {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        println!("{:?}", uuid.as_bytes());
        // let id = std::str::from_utf8(uuid.as_bytes()).expect("Failed converting");
        let id = uuid.to_hyphenated().to_string();
        Self {
            id: id.to_string(),
            username: id.to_string(),
            x: 0.0,
            y: 0.0,
            char_code: 1,
        }
    }
}

pub struct PlayerType {
    pub player: Handle<ColorMaterial>,
}
