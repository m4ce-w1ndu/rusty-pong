use bevy::prelude::*;

/// Left or right paddle field
#[derive(Copy, Clone, PartialEq)]
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
    pub field: Field,
    pub score: Score
}

/// Ball.
/// 
/// This is the classic Pong ball.
#[derive(Component)]
pub struct Ball {
    pub speed: f32,
    pub visible: bool
}

/// Score.
/// 
/// This component keeps the score and is tied with the scoreboard
/// that will be printed on each side of the field.
#[derive(Component)]
pub struct Score {
    pub value: i32
}
