use bevy::prelude::*;
use bevy::math::primitives::*;

#[derive(Component, Debug, Default)]
pub struct Position(pub Vec2);

pub const BALL_RADIUS: f32 = 2.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let ball = meshes.add(Sphere::new(BALL_RADIUS));
    let ball_color = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: ball.clone(),
        material: ball_color.clone(),
        ..default()
    })
    .insert(Position(Vec2::ZERO));
}
