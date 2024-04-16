use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};

use crate::components::{
    BallBundle,
    FieldLimitBundle,
    PaddleBundle,
    Player,
    PlayerType,
    BALL_SIZE,
    FIELD_LIMIT_HEIGHT,
    PADDLES_PADDING,
    PADDLE_HEIGHT,
    PADDLE_WIDTH
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn_ball,
            spawn_paddles,
            spawn_field_limits
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
        },
    ));
}

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let field_width = window.resolution.width();
        let padding = PADDLES_PADDING;
        let right_paddle_x = field_width / 2.0 - padding;
        let left_paddle_x = -field_width / 2.0 + padding;

        let paddle_mesh = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));

        commands.spawn((
            Player::new(PlayerType::Player),
            PaddleBundle::new(right_paddle_x, 0.0),
            MaterialMesh2dBundle {
                mesh: meshes.add(paddle_mesh.clone()).into(),
                material: materials.add(Color::WHITE),
                ..default()
            },
        ));

        commands.spawn((
            Player::new(PlayerType::Ai),
            PaddleBundle::new(left_paddle_x, 0.0),
            MaterialMesh2dBundle {
                mesh: meshes.add(paddle_mesh.clone()).into(),
                material: materials.add(Color::WHITE),
                ..default()
            },
        ));
    }
}

fn spawn_field_limits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let width = window.resolution.width();
        let height = window.resolution.height();

        let top_limit_y = height / 2.0 - FIELD_LIMIT_HEIGHT / 2.0;
        let bottom_limit_y = -height / 2.0 - FIELD_LIMIT_HEIGHT / 2.0;

        let top_limit = FieldLimitBundle::new(
            0.0,
            top_limit_y,
            width
        );
        let bottom_limit = FieldLimitBundle::new(
            0.0,
            bottom_limit_y,
            width
        );

        commands.spawn((
            top_limit.clone(),
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::from_size(top_limit.shape.0)).into(),
                material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.0))),
                ..default()
            }
        ));

        commands.spawn((
            bottom_limit.clone(),
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::from_size(bottom_limit.shape.0)).into(),
                material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.0))),
                ..default()
            }
        ));
    }
}
