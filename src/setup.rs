use bevy::{
    app::Plugin, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::{PresentMode, Window, WindowPlugin, WindowTheme}, DefaultPlugins
};

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
    commands.spawn(MaterialMesh2dBundle {
        mesh: left_paddle,
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(-paddle_x_start, paddle_y_start, 0.0),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: right_paddle,
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(paddle_x_start, paddle_y_start, 0.0),
        ..default()
    });
}
