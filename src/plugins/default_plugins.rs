use bevy::prelude::*;

pub struct CustomDefaultPlugin;

impl Plugin for CustomDefaultPlugin {
    fn build(&self, app: &mut App) {
        let window = get_window();
        let default_plugins = DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(window),
                ..default()
            })
            .build();

        app.add_plugins(default_plugins);
    }
}

fn get_window() -> Window {
    Window {
        title: "Fred's Revenge".to_string(),
        resolution: (800.0, 600.0).into(),
        resizable: true,
        ..default()
    }
}
