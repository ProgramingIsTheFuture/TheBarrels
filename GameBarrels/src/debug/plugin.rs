use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::player::types::Player;

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
            .add_startup_system(start_text.system())
            .add_system(set_text.system());
    }
}

fn start_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());

    let debug_strings = DebugStrings::new();

    commands
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
        .insert(debug_strings);
}

fn set_text(
    diagnostics: Res<Diagnostics>,
    player: Res<Player>,
    mut query: Query<(&mut Text, &mut DebugStrings)>,
) {
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
