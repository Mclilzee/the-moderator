use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plugins::default_plugins::CustomDefaultPlugin;
use plugins::player_plugin::PlayerPlugin;
use plugins::spammer_plugin::SpammerPlugins;
mod components;
mod plugins;
mod resources;

fn main() {
    App::new()
        .add_plugins(CustomDefaultPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SpammerPlugins)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 800.0,
        min_height: 250.0,
    };
    commands.spawn(camera);
}
