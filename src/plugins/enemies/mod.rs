use bevy::prelude::*;
mod spammer_plugin;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spammer_plugin::SpammerPlugin);
    }
}
