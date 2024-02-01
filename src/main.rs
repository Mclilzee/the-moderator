use bevy::prelude::*;
use plugins::hello_plugin::HelloPlugin;
mod plugins;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
