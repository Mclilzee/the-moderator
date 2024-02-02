use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plugins::player_plugin::PlayerPlugin;
mod components;
mod plugins;
mod resources;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
