use bevy::prelude::*;

/// Size of a Ball (radius)
pub const BALL_SIZE: f32 = 5.0;

pub const PADDLE_SPEED: f32 = 1.0;
pub const PADDLE_WIDTH: f32 = 14.0;
pub const PADDLE_HEIGHT: f32 = 56.0;

pub const PADDLES_PADDING: f32 = 22.0;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PlayerType {
    Ai,
    Player
}

/// Position of an entity in the Pong game.
#[derive(Component)]
pub struct Position(pub Vec2);

/// Velocity of an entity.
#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Shape(pub Vec2);

/// Ball component.
/// 
/// This will be added to a bundle, so it is not made fully public.
#[derive(Component)]
pub struct Ball;

/// Player is defined as a component with a player type.
#[derive(Component)]
pub struct Player {
    player_type: PlayerType
}

impl Player {
    pub fn new(player_type: PlayerType) -> Player {
        Player {
            player_type
        }
    }
}

/// BallBundle is a bundle that comprises a Ball and its position.
#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    shape: Shape,
    velocity: Velocity,
    position: Position
}

impl BallBundle {
    pub fn new(x: f32, y: f32) -> BallBundle {
        BallBundle {
            ball: Ball,
            shape: Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
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
    shape: Shape,
    position: Position,
    velocity: Velocity
}

impl PaddleBundle {
    pub fn new(x: f32, y: f32) -> PaddleBundle {
        PaddleBundle {
            paddle: Paddle,
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            position: Position(Vec2::new(x, y)),
            velocity: Velocity(Vec2::new(0.0, 0.0))
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
