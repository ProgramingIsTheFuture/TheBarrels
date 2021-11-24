use bevy::prelude::*;

use crate::global_state::entities::StateStruct;
use crate::menu::plugin::GameButton;
use crate::windows::status::WindowStatus;

pub struct ButtonColors {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
}

impl FromWorld for ButtonColors {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        Self {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.30, 0.15, 0.50).into()),
        }
    }
}

pub fn game_button_update(
    btn: Res<ButtonColors>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<GameButton>),
    >,
    mut text_query: Query<&mut Text>,
    state: Res<StateStruct>,
    mut window_state: ResMut<State<WindowStatus>>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                window_state.set(WindowStatus::InGameWindow);
            }
            Interaction::Hovered => {
                match state.player {
                    Some(_) => text.sections[0].value = "Continue".to_string(),
                    None => text.sections[0].value = "New game".to_string(),
                }
                *material = btn.hovered.clone();
            }
            Interaction::None => {
                match state.player {
                    Some(_) => text.sections[0].value = "Continue".to_string(),
                    None => text.sections[0].value = "New game".to_string(),
                }
                *material = btn.normal.clone();
            }
        }
    }
}
