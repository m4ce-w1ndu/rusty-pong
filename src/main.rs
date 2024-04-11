mod setup;
mod entities;
mod input;

use input::player_input_system;
use setup::{AppConfiguration, DrawField};
use bevy::{
    prelude::*
};


fn main() {
    App::new()
        .add_plugins(AppConfiguration)
        .add_plugins(DrawField)
        .add_systems(Update, player_input_system)
        .run();
}