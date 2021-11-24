use crate::network::plugin::Network;
use crate::player::types::Player;
use bevy::prelude::Entity;

#[derive(Debug)]
pub struct EntitiesController {
    pub entities: Vec<Entity>,
}

#[derive(Debug)]
pub struct MenuEntitiesController {
    pub entities: Vec<Entity>,
}

pub struct StateStruct {
    pub player: Option<Player>,
    pub network: Option<Network>,
}

impl Default for StateStruct {
    fn default() -> Self {
        Self {
            player: None,
            network: None,
        }
    }
}
