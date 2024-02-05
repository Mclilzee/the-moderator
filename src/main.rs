use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use components::Player;
use plugins::default_plugins::CustomDefaultPlugin;
use plugins::player_plugin::PlayerPlugin;
use plugins::spammer_plugin::SpammerPlugins;
mod components;
mod consts;
mod plugins;
mod resources;

#[derive(Clone, PartialEq, Eq, Debug, Hash, SystemSet)]
pub enum InGameSet {
    Movement,
    UserInput,
}

fn main() {
    App::new()
        .configure_sets(Update, (InGameSet::UserInput, InGameSet::Movement))
        .add_plugins(CustomDefaultPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SpammerPlugins)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, follow_player)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 600.0,
        min_height: 300.0,
    };
    commands.spawn(camera);
}

fn follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let mut translation = player_query.single().translation;
    translation.y += 50.0;

    camera_transform.translation = translation;
}
