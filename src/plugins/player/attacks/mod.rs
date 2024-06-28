use bevy::prelude::*;

mod hammer_throw;

pub struct AttacksPlugin;

impl Plugin for AttacksPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(hammer_throw::HammerPlugin);
    }
}
