use bevy::prelude::*;

use crate::components::Player;

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
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(2.0, 2.0)),
                ..default()
            },
            ..default()
        },
        Name::new("Player Boundary"),
        DebugBox,
    ));
}

type WithDebugBox = (With<DebugBox>, Without<Player>);
type WithPlayer = (With<Player>, Without<DebugBox>);

fn follow_player_shape(
    player: Query<&Transform, WithPlayer>,
    mut debug_box: Query<&mut Transform, WithDebugBox>,
) {
    let player_transform = player.single();
    let mut box_trasnform = debug_box.single_mut();
    box_trasnform.translation = player_transform.translation;
}
