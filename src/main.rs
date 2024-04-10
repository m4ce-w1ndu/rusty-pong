mod setup;
mod entities;

use setup::{AppConfiguration, DrawField};
use bevy::{
    prelude::*
};


fn main() {
    App::new()
        .add_plugins(AppConfiguration)
        .add_plugins(DrawField)
        .run();
}