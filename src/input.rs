use bevy::prelude::*;

use crate::{
    entities::{Field, Paddle},
    setup::{PADDLE_HEIGHT, PADDLE_Y_OFFSET, SCREEN_HEIGHT}
};

/// Paddle movement speed
pub const PADDLE_Y_SPEED: f32 = 4.0;

pub fn player_input_system(
    _time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>
) {
    let results = query.iter_mut();

    for (paddle, mut transform) in results {
        if keyboard_input.pressed(KeyCode::KeyW) && paddle.field == Field::Left {
            if paddle_is_in_boundaries(transform.translation.y + PADDLE_Y_SPEED) {
                transform.translation.y += PADDLE_Y_SPEED;
            }
        }
    
        if keyboard_input.pressed(KeyCode::KeyS) && paddle.field == Field::Left {
            if paddle_is_in_boundaries(transform.translation.y - PADDLE_Y_SPEED) {
                transform.translation.y -= PADDLE_Y_SPEED;
            }
        }
    
        if keyboard_input.pressed(KeyCode::KeyO) && paddle.field == Field::Right {
            if paddle_is_in_boundaries(transform.translation.y + PADDLE_Y_SPEED) {
                transform.translation.y += PADDLE_Y_SPEED;
            }
        }
    
        if keyboard_input.pressed(KeyCode::KeyL) && paddle.field == Field::Right {
            if paddle_is_in_boundaries(transform.translation.y - PADDLE_Y_SPEED) {
                transform.translation.y -= PADDLE_Y_SPEED;
            }
        }
    }
}

fn paddle_is_in_boundaries(y_position: f32) -> bool {
    let min_pos = -((SCREEN_HEIGHT / 2.0) - (PADDLE_HEIGHT / 2.0) - PADDLE_Y_OFFSET);
    let max_pos = -min_pos;
    y_position > min_pos && y_position < max_pos
}