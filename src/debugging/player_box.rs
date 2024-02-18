use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::components::HitBox;

#[derive(Component)]
struct DebugBox;

#[derive(Resource)]
struct DebugState {
    on: bool,
}

pub struct PlayerBoxPlugin;

impl Plugin for PlayerBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_debug_boxes)
            .add_systems(Update, dispawn_debug_boxes)
            .add_systems(
                Update,
                toggle_state.run_if(input_just_pressed(KeyCode::ControlLeft)),
            );
    }
}

fn toggle_state(mut state: ResMut<DebugState>) {
    state.on = !state.on;
}

fn spawn_debug_boxes(
    mut commands: Commands,
    hitbox_query: Query<(Entity, &HitBox)>,
    debugbox_query: Query<Entity, With<DebugBox>>,
    debug_state: Res<DebugState>,
) {
    if debugbox_query.get_single().is_ok() || !debug_state.on {
        return;
    }
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

fn dispawn_debug_boxes(
    mut commands: Commands,
    query: Query<Entity, With<DebugBox>>,
    debug_state: Res<DebugState>,
) {
    if debug_state.on {
        return;
    }

    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
