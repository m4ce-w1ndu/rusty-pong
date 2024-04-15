use bevy::prelude::*;

/// Position of an entity in the Pong game.
#[derive(Component)]
pub struct Position(pub Vec2);

/// Velocity of an entity.
#[derive(Component)]
pub struct Velocity(pub Vec2);

/// Ball component.
/// 
/// This will be added to a bundle, so it is not made fully public.
#[derive(Component)]
pub struct Ball;

/// BallBundle is a bundle that comprises a Ball and its position.
#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    velocity: Velocity,
    position: Position
}

impl BallBundle {
    /// Size of a Ball (radius)
    pub const BALL_SIZE: f32 = 5.0;

    pub fn new(x: f32, y: f32) -> BallBundle {
        BallBundle {
            ball: Ball,
            velocity: Velocity(Vec2::new(x, y)),
            position: Position(Vec2::new(0.0, 0.0))
        }
    }
}

/// Game paddle.
#[derive(Component)]
pub struct Paddle;

/// Paddle bundle.
/// 
/// Combines a paddle and its position.
#[derive(Bundle)]
pub struct PaddleBundle {
    paddle: Paddle,
    position: Position
}

impl PaddleBundle {
    pub const PADDLE_SPEED: f32 = 1.0;
    pub const PADDLE_WIDTH: f32 = 14.0;
    pub const PADDLE_HEIGHT: f32 = 56.0;

    pub fn new(x: f32, y: f32) -> PaddleBundle {
        PaddleBundle {
            paddle: Paddle,
            position: Position(Vec2::new(x, y))
        }
    }
}

/// Spawns an Ortho camera on the playing field.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

/// Spawn the camera onto the playing field.
fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn_empty()
        .insert(Camera2dBundle::default());
}
