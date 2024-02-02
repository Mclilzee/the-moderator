use bevy::prelude::*;

use crate::components::{Hp, Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    let player_sprite = SpriteBundle {
        transform: Transform::default(),
        visibility: Visibility::Visible,
        ..default()
    };
    commands.spawn((player_sprite, Player, Hp(100)));
}
