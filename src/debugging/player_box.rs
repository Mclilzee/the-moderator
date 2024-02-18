use bevy::prelude::*;

use crate::components::{HitBox, Player};

#[derive(Component)]
struct DebugBox;

pub struct PlayerBoxPlugin;

impl Plugin for PlayerBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_hitboxes)
            .add_systems(Update, dispawn_hitboxes);
    }
}

fn spawn_hitboxes(
    mut commands: Commands,
    query: Query<(Entity, &HitBox)>,
    keys: Res<Input<KeyCode>>,
) {
    if !keys.pressed(KeyCode::ControlLeft) || !keys.pressed(KeyCode::H) {
        return;
    }

    info!("Pressed for debugging");
    for (parent, hitbox) in query.iter() {
        let child = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(hitbox.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("HitBox Debug"),
                DebugBox,
            ))
            .id();

        commands.entity(parent).add_child(child);
    }
}

fn dispawn_hitboxes(mut commands: Commands, query: Query<(Entity, &DebugBox)>) {}
