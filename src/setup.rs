use bevy::{
    prelude::*,
    DefaultPlugins,
    app::Plugin,
    window::{WindowPlugin, Window, PresentMode, WindowTheme},
};

pub struct AppConfiguration;

impl Plugin for AppConfiguration {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Pong".into(),
                name: Some("rusty.pong".into()),
                resolution: (828.0, 525.0).into(),
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }), ..default()
        }));
    }
}