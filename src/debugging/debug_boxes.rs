use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::components::BoundaryBox;

#[derive(Component)]
struct DebugBox;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum BoxDebugState {
    #[default]
    Off,
    On,
}

pub struct DebugBoxPlugin;

impl Plugin for DebugBoxPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BoxDebugState>()
            .add_systems(
                Update,
                toggle_state.run_if(input_just_pressed(KeyCode::ControlLeft)),
            )
            .add_systems(OnEnter(BoxDebugState::On), spawn_debug_boxes)
            .add_systems(OnExit(BoxDebugState::On), dispawn_debug_boxes);
    }
}

fn toggle_state(mut next: ResMut<NextState<BoxDebugState>>, current: Res<State<BoxDebugState>>) {
    info!("Current state: {:?}", current);
    match current.get() {
        BoxDebugState::On => next.set(BoxDebugState::Off),
        BoxDebugState::Off => next.set(BoxDebugState::On),
    }
}

fn spawn_debug_boxes(mut commands: Commands, boundary_box_query: Query<(Entity, &BoundaryBox)>) {
    for (parent, boundary_box) in boundary_box_query.iter() {
        let child = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(boundary_box.boundary),
                        ..default()
                    },
                    ..default()
                },
                Name::new("BoundaryBox Debug"),
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
