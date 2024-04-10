mod setup;

use setup::AppConfiguration;
use bevy::{
    prelude::*
};


fn main() {
    App::new()
        .add_plugins(AppConfiguration)
        .run();
}