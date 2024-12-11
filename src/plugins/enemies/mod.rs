use bevy::prelude::*;
mod spammer;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spammer::SpammerPlugin);
    }
}
