const HAMMER_SPEED: f32 = 20.0;

use crate::{
    components::{EntityState, Player},
    plugins::{
        asset_loader::{AnimationKey, AnimationMap},
        default_plugins::CursorDirection,
    },
    AnimationTimer,
};
use bevy::prelude::*;
use bevy_rapier2d::{dynamics::Velocity, geometry::Collider};

#[derive(Component)]
struct Hammer;

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_button_input)
            .add_systems(Update, animate);
    }
}

fn mouse_button_input(
    mut command: Commands,
    player_transform: Query<&Transform, With<Player>>,
    direction: Res<CursorDirection>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        command.spawn((
            Hammer,
            EntityState::Idle,
            SpriteSheetBundle {
                transform: *player_transform.single(),
                ..default()
            },
            Collider::cuboid(10.0, 10.0),
            Velocity::linear(direction.0 * HAMMER_SPEED),
        ));
    }
}

fn animate(
    mut sprite_query: Query<(&mut TextureAtlas, &mut Sprite, &EntityState), With<Hammer>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
    animation: Res<AnimationMap>,
) {
    let (mut atlas, mut sprite, state) = sprite_query.single_mut();

    let hammer_animation = &animation
        .0
        .get(&AnimationKey::Hammer)
        .expect("Animation were not found");

    let frames = hammer_animation
        .indices
        .get(state)
        .unwrap_or(&hammer_animation.default);

    timer.0.tick(time.delta());
    if timer.0.finished() {
        let mut index = atlas.index + 1;

        if atlas.index >= frames.last_frame || atlas.index < frames.first_frame {
            index = frames.first_frame;
        }

        atlas.index = index;
    }
}
