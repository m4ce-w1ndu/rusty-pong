use bevy::{
    prelude::*,
    app::Plugin, 
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::{PresentMode, Window, WindowPlugin, WindowTheme},
    DefaultPlugins
};

use crate::entities::{Ball, Field, Paddle};

/// Configuration plugin.
/// 
/// This plugin sets up the application, rendering the first
/// 2D shapes that will make the playing field for the game.
pub struct AppConfiguration;

/// Field drawing plugin.
/// 
/// This plugin is responsible of drawing the initial appearance
/// of the playing field, and tying the different entities and
/// components together.
pub struct DrawField;

/// Default Pong screen width
pub const SCREEN_WIDTH: f32 =  828.0;

/// Default Pong screen height
pub const SCREEN_HEIGHT: f32 = 525.0;

/// Pong paddle width
pub const PADDLE_WIDTH: f32 = 14.0;

/// Pong paddle height
pub const PADDLE_HEIGHT: f32 = 56.0;

/// Pong paddle X-axis border offset
pub const PADDLE_X_OFFSET: f32 = 10.0;

/// Pong paddle Y-axis border offset
pub const PADDLE_Y_OFFSET: f32 = 10.0;

/// Pong ball radius
pub const BALL_RADIUS: f32 = 7.0;

impl Plugin for AppConfiguration {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Pong".into(),
                name: Some("rusty.pong".into()),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }), ..default()
        }));
    }
}

impl Plugin for DrawField {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_initial_configuration);
    }
}

fn draw_initial_configuration(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    // Add a default 2D Orthogonal camera
    commands.spawn(Camera2dBundle::default());
    
    // Draw paddles
    draw_paddles(&mut commands, &mut meshes, &mut materials);
    // Draw ball
    draw_ball(&mut commands, &mut meshes, &mut materials);
}

fn draw_paddles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>) {

    // Create paddles
    let left_paddle = Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT)));
    let right_paddle = Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT)));

    // Paddles start positions
    let paddle_x_start = (SCREEN_WIDTH / 2.0) - (PADDLE_WIDTH / 2.0) - PADDLE_X_OFFSET;
    let paddle_y_start = (SCREEN_HEIGHT / 2.0) - (PADDLE_HEIGHT / 2.0) - PADDLE_Y_OFFSET;

    // Spawn the paddles
    commands.spawn((MaterialMesh2dBundle {
        mesh: left_paddle,
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(-paddle_x_start, paddle_y_start, 0.0),
        ..default()
    }, Paddle {
        position: Vec2::new(-paddle_x_start, paddle_y_start),
        field: Field::Left
    }));

    commands.spawn((MaterialMesh2dBundle {
        mesh: right_paddle,
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(paddle_x_start, paddle_y_start, 0.0),
        ..default()
    }, Paddle {
        position: Vec2::new(paddle_x_start, paddle_y_start),
        field: Field::Right
    }));

    
}

fn draw_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>) {

    // Create the ball
    let ball = Mesh2dHandle(meshes.add(Circle::new(BALL_RADIUS)));

    // Spawn the ball
    commands.spawn((MaterialMesh2dBundle {
        mesh: ball,
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        visibility: Visibility::Hidden,
        ..default()
    }, Ball {
        position: Vec2::new(0.0, 0.0),
        speed: 0.0,
        visible: false
    }));
}
