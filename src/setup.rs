use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};

use crate::components::{
    BALL_SIZE,
    PADDLE_HEIGHT,
    PADDLE_WIDTH,
    BallBundle,
    PaddleBundle
};

pub const INIT_PADDLE_X: f32 = 20.0;
pub const INIT_PADDLE_Y: f32 = -25.0;

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
    // Add the mesh and material into memory
    let mesh_handle = meshes.add(
        Mesh::from(Circle::new(BALL_SIZE))
    );
    let material_handle = materials.add(
        ColorMaterial::from(Color::rgb(1.0, 0.0, 0.0))
    );

    // Spawn the ball
    commands.spawn((
        BallBundle::new(1.0, 0.0),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        }
    ));
}

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let mesh = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
    let material = ColorMaterial::from(Color::rgb(0.0, 1.0, 0.0));

    commands.spawn((
        PaddleBundle::new(INIT_PADDLE_X, INIT_PADDLE_Y),
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(material),
            ..default()
        }
    ));
}
