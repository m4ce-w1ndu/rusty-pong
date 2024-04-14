use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

use crate::{entities::{Field, Paddle}, setup::{PADDLE_HEIGHT, PADDLE_WIDTH}};

pub const BALL_SIZE: f32 = 16.0;

pub const BALL_SPEED: f32 = 4.0;

#[derive(Component)]
pub struct Ball;

pub struct BallPlugin;

#[derive(Debug)]
struct MinMax {
    y_min: f32,
    y_max: f32
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_ball_position);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let ball = Mesh2dHandle(meshes.add(Rectangle::new(BALL_SIZE, BALL_SIZE)));
    let ball_color = materials.add(Color::WHITE);

    commands.spawn((MaterialMesh2dBundle {
        mesh: ball,
        material: ball_color,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        visibility: Visibility::Visible,
        ..default()
    }, Ball));
}

fn update_ball_position(
    mut ball: Query<&mut Transform, With<Ball>>,
    mut paddle: Query<(&mut Transform, &mut Paddle), Without<Ball>>
) {
    let mut ball_position = ball.single_mut();

    static mut VELOCITY: Vec2 = Vec2::new(BALL_SPEED, 0.0);

    let paddles = paddle.iter_mut();
    for (transform, paddle) in paddles {

        let paddle_range = get_paddle_range(transform.translation.y);
        println!("{:#?}", paddle_range);

        match paddle.field {
            Field::Left =>
            if ball_position.translation.x <= (transform.translation.x + PADDLE_WIDTH) &&
               ball_position.translation.y < paddle_range.y_max && ball_position.translation.y > paddle_range.y_min {
                unsafe { VELOCITY.x = -VELOCITY.x; };
                unsafe { VELOCITY.y = get_bounce_angle(ball_position.translation.y, transform.translation.y) };
            },
            Field::Right =>
            if ball_position.translation.x >= (transform.translation.x - PADDLE_WIDTH) &&
               ball_position.translation.y < paddle_range.y_max && ball_position.translation.y > paddle_range.y_min {
                unsafe { VELOCITY.x = -VELOCITY.x; };
                unsafe { VELOCITY.y = get_bounce_angle(ball_position.translation.y, transform.translation.y) };
            }
        }
    }

    // unsafe { ball_position.translation.x += BALL_VELOCITY * DIRECTION };
    unsafe { ball_position.translation += Vec3::new(VELOCITY.normalize().x, VELOCITY.normalize().y, 0.0) }
}

fn get_paddle_range(y_pos: f32) -> MinMax {
    MinMax {
        y_min: y_pos - PADDLE_HEIGHT / 2.0,
        y_max: y_pos + PADDLE_HEIGHT / 2.0
    }
}

fn get_bounce_angle(ball_y: f32, paddle_y: f32) -> f32 {
    let hit_position = ((paddle_y + PADDLE_HEIGHT) - ball_y) as i32;

    match hit_position {
        1..=8 => 3.0,
        9..=16 => 2.0,
        17..=24 => 1.0,
        25..=32 => 0.0,
        33..=40 => -1.0,
        41..=48 => -2.0,
        49..=56 => -3.0,
        _ => 0.0
    }
}
