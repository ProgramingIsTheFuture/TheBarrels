use bevy::prelude::*;

use crate::global_state::entities::MenuEntitiesController;
use crate::windows::status::WindowStatus;

use crate::menu::button::*;

pub struct GameButton {}

pub struct MainMenu {}

impl Plugin for MainMenu {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonColors>()
            .add_system_set(
                SystemSet::on_enter(WindowStatus::MenuWindow).with_system(setup_menu.system()),
            )
            .add_system(game_button_update.system());
    }
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    btn: Res<ButtonColors>,
    mut entities: ResMut<MenuEntitiesController>,
) {
    let ent = commands.spawn_bundle(UiCameraBundle::default()).id();
    entities.entities.push(ent);
    let ent = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: btn.normal.clone(),
            ..Default::default()
        })
        .insert(GameButton {})
        .with_children(|parent| {
            let ent = parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .id();
            entities.entities.push(ent);
        })
        .id();
    entities.entities.push(ent);

    let ent = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect {
                    top: Val::Px(400.0),
                    left: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Auto,
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: btn.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            let ent = parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Settings",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .id();
            entities.entities.push(ent);
        })
        .id();
    entities.entities.push(ent);
}
