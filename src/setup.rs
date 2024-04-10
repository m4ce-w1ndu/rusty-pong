use bevy::{
    prelude::*,
    DefaultPlugins,
    app::Plugin,
    window::{WindowPlugin, Window, PresentMode, WindowTheme},
};

/// Configuration plugin.
/// 
/// This plugin sets up the application, rendering the first
/// 2D shapes that will make the playing field for the game.
pub struct AppConfiguration;

/// Default Pong screen width
pub const SCREEN_WIDTH: f32 =  828.0;

/// Default Pong screen height
pub const SCREEN_HEIGHT: f32 = 525.0;

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