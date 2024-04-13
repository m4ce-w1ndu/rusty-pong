use bevy::prelude::*;

use crate::{entities::Score, setup::SCREEN_WIDTH};

/// Plugin to draw the scoreboard on the playing field
pub struct ScoreboardPlugin;

/// Size of scoreboard text font
pub const SCOREBOARD_TEXT_SIZE: f32 = 78.0;

/// Scoreboard font to be loaded from assets
pub const SCOREBOARD_FONT: &'static str = "fonts/prstartk.ttf";

/// Scoreboard top margin
const SCOREBOARD_TOP_MARGIN: f32 = 10.0;

/// Scoreboard horizontal margin
const SCOREBOARD_HORI_MARGIN: f32 = (SCREEN_WIDTH / 4.0) - (SCOREBOARD_TEXT_SIZE / 2.0);

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

/// Draws the scoreboard on the playing field
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((TextBundle::from_section(
        "0",
        TextStyle {
            font: asset_server.load(SCOREBOARD_FONT),
            font_size: SCOREBOARD_TEXT_SIZE,
            ..default()
        })
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(SCOREBOARD_TOP_MARGIN),
            left: Val::Px(SCOREBOARD_HORI_MARGIN),
            ..default()
        }),
        Score { value: 0 }
    ));

    commands.spawn((TextBundle::from_section(
        "0",
        TextStyle {
            font: asset_server.load(SCOREBOARD_FONT),
            font_size: SCOREBOARD_TEXT_SIZE,
            ..default()
        })
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(SCOREBOARD_TOP_MARGIN),
            right: Val::Px(SCOREBOARD_HORI_MARGIN),
            ..default()
        }),
        Score { value: 0 }
    ));
}
