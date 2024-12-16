use bevy::prelude::*;
mod spammer;
mod flying_spammer;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spammer::SpammerPlugin).add_plugins(flying_spammer::FlyingSpammerPlugin);
    }
}
