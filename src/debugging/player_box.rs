use bevy::prelude::*;

use crate::components::{HitBox, Player};

#[derive(Component)]
struct DebugBox;

pub struct PlayerBoxPlugin;

impl Plugin for PlayerBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_box)
            .add_systems(Update, follow_player_shape);
    }
}

fn spawn_box(mut commands: Commands) {
    commands.spawn((
        SpriteBundle::default(),
        Name::new("Player Boundary"),
        DebugBox,
    ));
}

type WithDebugBox = (With<DebugBox>, Without<Player>);
type WithPlayer = (With<Player>, Without<DebugBox>);

fn follow_player_shape(
    player: Query<(&Transform, &HitBox), WithPlayer>,
    mut debug_box: Query<(&mut Sprite, &mut Transform), WithDebugBox>,
) {
    let (player_transform, player_hitbox) = player.single();
    let (box_sprite, mut box_trasnform) = debug_box.single_mut();
    box_trasnform.translation = player_transform.translation;

    if let Some(mut size) = box_sprite.custom_size {
        size.x = player_hitbox.0.x;
        size.y = player_hitbox.0.y;
    }
}
