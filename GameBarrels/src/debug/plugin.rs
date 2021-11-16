use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::global_state::entities::{EntitiesController, StateStruct};
use crate::windows::status::WindowStatus;

pub struct DebugStrings {
    fps: f32,
    x: f32,
    y: f32,
}

impl DebugStrings {
    fn to_string(&self) -> String {
        format!(
            "[FPS]: {:.1}\nX: {:.2}\nY: {:.2}\n",
            self.fps, self.x, self.y
        )
    }

    fn new() -> DebugStrings {
        DebugStrings {
            fps: 0.0,
            x: 0.0,
            y: 0.0,
        }
    }
}

pub struct DebugPlugin {}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_system_set(
                SystemSet::on_enter(WindowStatus::InGameWindow).with_system(start_text.system()),
            )
            .add_system(set_text.system());
    }
}

// Function to spwan the debug text on the screen
fn start_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut entities: ResMut<EntitiesController>,
) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());

    // Initialize the debug string (fps: 0, x: 0, y: 0)
    let debug_strings = DebugStrings::new();

    let ent = commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                debug_strings.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 25.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(debug_strings)
        .id();

    entities.entities.push(ent);
}

// This function set the value of the string
// this will update the debug text with user info and with the FPS
fn set_text(
    diagnostics: Res<Diagnostics>,
    // player: Res<Player>,
    state: Res<StateStruct>,
    mut query: Query<(&mut Text, &mut DebugStrings)>,
) {
    let player = match state.player.clone() {
        Some(v) => v,
        None => return,
    };
    for (mut text, mut debug_str) in query.iter_mut() {
        let mut fps: f32 = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg as f32;
            }
        }

        debug_str.fps = fps;
        debug_str.x = player.x;
        debug_str.y = player.y;

        text.sections[0].value = debug_str.to_string();
    }
}
