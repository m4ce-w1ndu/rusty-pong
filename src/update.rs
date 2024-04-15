use bevy::{
    prelude::*,
    math::bounding::{
        Aabb2d,
        BoundingCircle,
        BoundingVolume,
        IntersectsVolume
    }
};

/// Type of collision detected by the bounding box system.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom
}

use crate::components::{
    Ball,
    Position,
    Velocity,
    Shape
};

pub struct UpdateGamePlugin;

impl Plugin for UpdateGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_positions,
            move_ball,
            read_collisions
        ));
    }
}

/// Updates the ball position based on its velocity
fn move_ball(
    mut ball: Query<(&mut Position, &Velocity), With<Ball>>
) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0;
    }
}

/// Updates all transforms based on the associted position.
fn update_positions(mut pos: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut pos {
        transform.translation = position.0.extend(0.0);
    }
}

fn read_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    others: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((
        mut ball_velocity,
        ball_position,
        ball_shape
    )) = ball.get_single_mut() {
        for (position, shape) in &others {
            if let Some(collision) = collide_with_field(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(
                    position.0,
                    shape.0 / 2.0,
                )
            ) {
                match collision {
                    Collision::Left => { ball_velocity.0.x *= -1.0; },
                    Collision::Right => { ball_velocity.0.x *= -1.0 },
                    Collision::Top => { ball_velocity.0.y *= -1.0 },
                    Collision::Bottom => { ball_velocity.0.y *= -1.0 }
                }
            }
        }
    }
}

/// Finds the impact position with the field (bounding box with field sides)
fn collide_with_field(ball: BoundingCircle, side: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&side) {
        return None;
    }

    let impact_point = side.closest_point(ball.center());
    let offset = ball.center() - impact_point;

    // Calculate the position of the impact
    let impact_position = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0.0 {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0.0 {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(impact_position)
}