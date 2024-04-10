use bevy::prelude::*;

/// Left or right paddle field
pub enum PaddleField {
    Left,
    Right
}

/// Paddle.
/// 
/// The main object used to play Pong. It is represented by a
/// vertical line.
#[derive(Component)]
pub struct Paddle {
    position: Vec2,
    field: PaddleField
}

/// Ball.
/// 
/// This is the classic Pong ball.
#[derive(Component)]
pub struct Ball {
    position: Vec2
}