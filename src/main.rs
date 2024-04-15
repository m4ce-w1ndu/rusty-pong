use bevy::prelude::*;
use components::Position;

use crate::components::BallBundle;

mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera))
        .add_systems(Update, (Position::update_positions))
        .run();
}

fn spawn_ball(
    mut commands: Commands,
) {
    println!("Spawning ball...");
    commands
        .spawn_empty()
        .insert(Transform::default())
        .insert(BallBundle::new());
}

fn spawn_camera(
    mut commands: Commands
) {
    commands
        .spawn_empty();
}