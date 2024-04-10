use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Pong".into(),
                name: Some("rusty.pong".into()),
                resolution: (828.0, 525.0).into(),
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }), ..default()
        }))
        .run();
}