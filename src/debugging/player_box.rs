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
        app.add_state::<BoxDebugState>()
            .add_systems(
                Update,
                toggle_state.run_if(input_just_pressed(KeyCode::ControlLeft)),
            )
            .add_systems(OnEnter(BoxDebugState::On), spawn_debug_boxes)
            .add_systems(OnExit(BoxDebugState::Off), dispawn_debug_boxes);
    }
}

fn toggle_state(mut state: ResMut<NextState<BoxDebugState>>) {
    if let Some(current) = state.0 {
        let next = if current == BoxDebugState::On {
            BoxDebugState::Off
        } else {
            BoxDebugState::On
        };

        state.set(next);
    }
}

fn spawn_debug_boxes(mut commands: Commands, hitbox_query: Query<(Entity, &HitBox)>) {
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
