use bevy::prelude::*;

use crate::components::{HitBox, Player};

#[derive(Component)]
struct DebugBox;

pub struct PlayerBoxPlugin;

impl Plugin for PlayerBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_boxes)
    }
}

fn debug_boxes_control(
    mut commands: Commands,
    hitbox_query: Query<(Entity, &HitBox)>,
    debugbox_query: Query<Entity, With<DebugBox>>,
    keys: Res<Input<KeyCode>>,
) {
    if !keys.pressed(KeyCode::ControlLeft) || !keys.pressed(KeyCode::H) {
        return;
    }

    info!("Pressed for debugging");
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

fn spawn_boxes(

fn dispawn_boxes(boxes: ) {
    for entity in boxes
}
