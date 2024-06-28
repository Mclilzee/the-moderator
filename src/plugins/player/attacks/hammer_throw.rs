const HAMMER_SPEED: f32 = 200.0;
const ROTATION_SPEED: f32 = 2.0;

use crate::{
    components::{EntityState, Player},
    plugins::{
        asset_loader::{AnimationKey, AnimationMap},
        default_plugins::CursorDirection,
    },
    AnimationTimer,
};
use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{RigidBody, Velocity},
    geometry::{Collider, Sensor},
};

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
    animation_map: Res<AnimationMap>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let animation = animation_map
            .0
            .get(&AnimationKey::Hammer)
            .expect("Player animation were not found");

        let mut sprite_sheet = SpriteSheetBundle {
            transform: *player_transform.single(),
            ..default()
        };

        sprite_sheet.texture = animation.texture.clone();
        sprite_sheet.atlas = TextureAtlas {
            layout: animation.atlas.clone(),
            index: 1,
        };

        command.spawn((
            Hammer,
            EntityState::Idle,
            Collider::cuboid(14.0, 14.0),
            Sensor,
            RigidBody::KinematicVelocityBased,
            Velocity::linear(direction.0 * HAMMER_SPEED),
            sprite_sheet,
        ));
    }
}

fn animate(
    mut sprite_query: Query<(&mut TextureAtlas, &mut Transform, &EntityState), With<Hammer>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
    animation: Res<AnimationMap>,
) {
    for (mut atlas, mut transform, state) in sprite_query.iter_mut() {
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

        transform.rotate_z(f32::to_radians(ROTATION_SPEED));
    }
}
