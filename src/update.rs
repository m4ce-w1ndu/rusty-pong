use bevy::prelude::*;

use crate::components::{
    Ball,
    Position,
    Velocity
};

pub struct UpdateGamePlugin;

impl Plugin for UpdateGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_positions,
            move_ball
        ));
    }
}

/// Updates the ball position based on its velocity
fn move_ball(
    mut ball: Query<(&mut Position, &Velocity), With<Ball>>
) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0;
    }
}

/// Updates all transforms based on the associted position.
fn update_positions(mut pos: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut pos {
        transform.translation = position.0.extend(0.0);
    }
}