use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::components::HitBox;

#[derive(Component)]
struct DebugBox;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum BoxDebugState {
    #[default]
    Off,
    On,
}

pub struct PlayerBoxPlugin;

impl Plugin for PlayerBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_state()
            .add_systems(Update, spawn_debug_boxes)
            .add_systems(Update, dispawn_debug_boxes)
    }
}

fn spawn_debug_boxes(
    mut commands: Commands,
    hitbox_query: Query<(Entity, &HitBox)>,
    debugbox_query: Query<Entity, With<DebugBox>>,
) {
    for (parent, hitbox) in hitbox_query.iter() {
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

fn dispawn_debug_boxes(mut commands: Commands, query: Query<Entity, With<DebugBox>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
