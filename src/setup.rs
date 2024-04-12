use bevy::{
    prelude::*,
    app::Plugin, 
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::{PresentMode, Window, WindowPlugin, WindowTheme},
    DefaultPlugins
};

use crate::entities::{Ball, Field, Paddle, Score};

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

/// Paddle X-axis start position
pub const PADDLE_X_START: f32 = (SCREEN_WIDTH / 2.0) - (PADDLE_WIDTH / 2.0) - PADDLE_X_OFFSET;

/// Paddle Y-axis start position
pub const PADDLE_Y_START: f32 = (PADDLE_HEIGHT / 2.0) - PADDLE_Y_OFFSET;

/// Pong ball radius
pub const BALL_RADIUS: f32 = 7.0;

/// Number of half field line dots
pub const HALF_LINE_DOTS: u8 = 40;

/// Size of half-field line dots
pub const HALF_DOT_SIZE: f32 = 7.0;

/// Stride between half-field line dots
pub const HALF_DOT_STRIDE: f32 = 20.0;

/// Plugin for app configuration (implementation)
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

// Plugin for field setup (implementation)
impl Plugin for DrawField {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Startup, draw_paddles)
            .add_systems(Startup, draw_ball)
            .add_systems(Startup, draw_half);
    }
}

/// Calls the tree of functions that sets up the playing field
fn setup_camera(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>) {

    // Add a default 2D Orthogonal camera
    commands.spawn(Camera2dBundle::default());
}

/// Draws the paddles on the playing field, binding the correct
/// components to the paddle meshes.
fn draw_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    // Create paddles
    let left_paddle = Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT)));
    let right_paddle = Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT)));

    // Spawn the paddles
    commands.spawn((MaterialMesh2dBundle {
        mesh: left_paddle,
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(-PADDLE_X_START, PADDLE_Y_START, 0.0),
        ..default()
    }, Paddle {
        field: Field::Left,
        score: Score { value: 0 }
    }));

    commands.spawn((MaterialMesh2dBundle {
        mesh: right_paddle,
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(PADDLE_X_START, PADDLE_Y_START, 0.0),
        ..default()
    }, Paddle {
        field: Field::Right,
        score: Score { value: 0 }
    }));

    
}

/// Draws the ball on to the playing field.
fn draw_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

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
        speed: 0.0,
        visible: false
    }));
}

/// Draws the playing field's half line.
fn draw_half(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
    
    // Starting position for the line drawing phase
    let mut y_pos = -SCREEN_HEIGHT / 2.0;

    // Create a single dot mesh
    let dot = Mesh2dHandle(meshes.add(Rectangle::new(HALF_DOT_SIZE, HALF_DOT_SIZE)));

    // Spawn all the dots
    for _ in 0..HALF_LINE_DOTS {
        commands.spawn(MaterialMesh2dBundle {
            mesh: dot.clone(),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, y_pos, 0.0),
            ..default()
        });

        y_pos += HALF_DOT_STRIDE;
    }
}
