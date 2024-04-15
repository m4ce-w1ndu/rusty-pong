use bevy::prelude::*;

mod components;
mod update;
mod setup;

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
            DefaultPlugins,
            UpdateGamePlugin,
            SetupPlugin,
            CameraPlugin)
        )
        .run();
}
