use bevy::prelude::*;
mod components;

use crate::components::{
    Position,
    BallBundle,
    CameraPlugin
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin))
        .add_systems(Update, (components::update_positions))
        .run();
}
