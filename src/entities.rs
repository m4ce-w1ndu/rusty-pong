use bevy::prelude::*;

/// Left or right paddle field
#[derive(Copy, Clone)]
pub enum Field {
    Left,
    Right
}

/// Paddle.
/// 
/// The main object used to play Pong. It is represented by a
/// vertical line.
#[derive(Component)]
pub struct Paddle {
    pub position: Vec2,
    pub field: Field
}

/// Ball.
/// 
/// This is the classic Pong ball.
#[derive(Component)]
pub struct Ball {
    pub position: Vec2,
    pub speed: f32,
    pub visible: bool
}

/// Score.
/// 
/// This component keeps the score and is tied with the scoreboard
/// that will be printed on each side of the field.
#[derive(Component)]
pub struct Score {
    pub value: i32,
    pub field: Field
}
