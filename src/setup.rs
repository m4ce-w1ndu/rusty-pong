use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};

use crate::components::{
    BallBundle,
    PaddleBundle,
    Player,
    PlayerType,
    BALL_SIZE,
    PADDLE_HEIGHT,
    PADDLE_WIDTH,
    PADDLES_PADDING
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn_ball,
            spawn_paddles
        ));
    }
}

/// Spawn the ball in the field
pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn the ball
    commands.spawn((
        BallBundle::new(1.0, 0.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Circle::new(BALL_SIZE))).into(),
            material: materials.add(Color::WHITE),
            ..default()
        }
    ));
}

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>
) {
    if let Ok(window) = window.get_single() {
        let field_width = window.resolution.width();
        let padding = PADDLES_PADDING;
        let right_paddle_x = field_width / 2.0 - padding;
        let left_paddle_x = -field_width / 2.0 + padding;

        let paddle_mesh = Mesh::from(Rectangle::new(
            PADDLE_WIDTH,
            PADDLE_HEIGHT
        ));

        commands.spawn((
            Player::new(PlayerType::Player),
            PaddleBundle::new(right_paddle_x, 0.0),
            MaterialMesh2dBundle {
                mesh: meshes.add(paddle_mesh.clone()).into(),
                material: materials.add(Color::WHITE),
                ..default()
            }
        ));

        commands.spawn((
            Player::new(PlayerType::Ai),
            PaddleBundle::new(left_paddle_x, 0.0),
            MaterialMesh2dBundle {
                mesh: meshes.add(paddle_mesh.clone()).into(),
                material: materials.add(Color::WHITE),
                ..default()
            }
        ));
    }
}
