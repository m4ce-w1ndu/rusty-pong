use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution}
};

mod components;
mod update;
mod setup;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 525.0;
const WINDOW_SCALING: f32 = 1.0;

use crate::{
    setup::SetupPlugin,
    update::{
        UpdateGamePlugin,
    },
    components::{
        CameraPlugin,
    }
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(
                        WINDOW_WIDTH,
                        WINDOW_HEIGHT
                    ).with_scale_factor_override(WINDOW_SCALING),
                    enabled_buttons: EnabledButtons {
                        minimize: true,
                        maximize: false,
                        close: true
                    },
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            UpdateGamePlugin,
            SetupPlugin,
            CameraPlugin)
        )
        .run();
}
