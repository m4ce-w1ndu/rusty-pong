use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

#[derive(Component, Debug, Default)]
pub struct Position(pub Vec2);

pub const BALL_RADIUS: f32 = 8.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let ball = Mesh2dHandle(meshes.add(Circle::new(BALL_RADIUS)));
    let ball_color = materials.add(Color::WHITE);

    commands.spawn(MaterialMesh2dBundle {
        mesh: ball,
        material: ball_color,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
    .insert(Position(Vec2::ZERO));
}
