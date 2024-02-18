use bevy::{input::common_conditions::input_toggle_active, prelude::*};

use crate::components::HitBox;

#[derive(Component)]
struct DebugBox;

pub struct PlayerBoxPlugin;

impl Plugin for PlayerBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_boxes.run_if(input_toggle_active(false, KeyCode::ControlLeft)),
        );
    }
}

fn toggle_boxes(
    mut commands: Commands,
    hitbox_query: Query<(Entity, &HitBox)>,
    debugbox_query: Query<Entity, With<DebugBox>>,
) {
    let mut respawn = true;

    for entity in debugbox_query.iter() {
        commands.entity(entity).despawn();
        respawn = false;
    }

    if !respawn {
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
