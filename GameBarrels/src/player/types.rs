use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub direction: i8,
    pub moving: bool,
    pub char_code: i8,
    pub username: String,
}

impl Default for Player {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        // let id = std::str::from_utf8(uuid.as_bytes()).expect("Failed converting");
        let id = uuid.to_hyphenated().to_string();
        Self {
            id: id.to_string(),
            username: id.to_string(),
            x: 0.0,
            y: 0.0,
            direction: 3,
            moving: false,
            char_code: 1,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Action<T> {
    pub action: String,
    pub data: T,
}
