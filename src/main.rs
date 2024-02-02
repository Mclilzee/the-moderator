use bevy::prelude::*;
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
        .run();
}
