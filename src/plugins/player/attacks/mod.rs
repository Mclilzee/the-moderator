use bevy::prelude::*;
mod hammer_throw;
mod fire_slash;

pub struct AttacksPlugin;

impl Plugin for AttacksPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(hammer_throw::HammerThrowPlugin)
        .add_plugins(fire_slash::FireSlashPlugin);
    }
}
