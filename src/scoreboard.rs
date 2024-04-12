use bevy::prelude::*;

use crate::{entities::Score, setup::SCREEN_WIDTH};

pub struct ScoreboardPlugin;

pub const SCOREBOARD_TEXT_SIZE: f32 = 96.0;

pub const SCOREBOARD_FONT: &'static str = "fonts/AtlantisInternational.ttf";

const SCOREBOARD_TOP_MARGIN: f32 = 10.0;

const SCOREBOARD_LEFT_MARGIN: f32 = (SCREEN_WIDTH / 4.0) - (SCOREBOARD_TEXT_SIZE / 2.0);

const SCOREBOARD_RIGHT_MARGIN: f32 = -SCOREBOARD_LEFT_MARGIN;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

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
            top: Val::Px(10.0),
            left: Val::Px((SCREEN_WIDTH / 4.0) - (SCOREBOARD_TEXT_SIZE / 2.0)),
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
            top: Val::Px(10.0),
            left: Val::Px(((SCREEN_WIDTH / 4.0) * 3.0) - (SCOREBOARD_TEXT_SIZE / 2.0)),
            ..default()
        }),
        Score { value: 0 }
    ));
}
