use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};

use crate::components::BallBundle;

/// Spawn the ball in the field
pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh::from(Circle::new(BallBundle::BALL_SIZE));
    let color = ColorMaterial::from(Color::rgb(1.0, 0.0, 0.0));

    // Add the mesh and material into memory
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    // Spawn the ball
    commands.spawn((
        BallBundle::new(0.0, 0.0),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        }
    ));
}