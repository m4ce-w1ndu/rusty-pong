use bevy::prelude::*;

/// Position of an entity in the Pong game.
#[derive(Component)]
pub struct Position(Vec2);

impl Position {
    /// Updates all transforms based on the associted position.
    /// 
    /// Arguments:
    /// * `pos`: positions query.
    pub fn update_positions(mut pos: Query<(&mut Transform, &Position)>) {
        for (mut transform, position) in &mut pos {
            transform.translation = position.0.extend(0.0);
        }
    }
}

/// Ball component.
/// 
/// This will be added to a bundle, so it is not made fully public.
#[derive(Component)]
pub struct Ball;

/// BallBundle is a bundle that comprises a Ball and its position.
#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    position: Position
}

impl BallBundle {
    pub fn new() -> BallBundle {
        BallBundle {
            ball: Ball,
            position: Position(Vec2::new(0.0, 0.0))
        }
    }
}
