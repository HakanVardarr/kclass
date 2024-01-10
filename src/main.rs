use bevy::prelude::*;
use kclass::plugins::{MeansPlugin, SamplePlugin, WindowEventPlugin};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "K Class".to_string(),
                    resolution: (1200., 900.0).into(),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            WindowEventPlugin,
            SamplePlugin::new(2_000),
            MeansPlugin::new(8),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}
