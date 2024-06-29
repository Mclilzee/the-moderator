const HAMMER_SPEED: f32 = 350.0;
const ROTATION_SPEED: f32 = 2.0;

use crate::{
    components::{EntityState, Player},
    plugins::{
        asset_loader::{AnimationKey, AnimationMap},
        default_plugins::CursorPosition,
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
    player: Query<&Transform, With<Player>>,
    cursor_position: Res<CursorPosition>,
    buttons: Res<ButtonInput<MouseButton>>,
    animation_map: Res<AnimationMap>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let animation = animation_map
            .0
            .get(&AnimationKey::Hammer)
            .expect("Player animation were not found");

        let p_transform = player.single();

        let mut sprite_sheet = SpriteSheetBundle {
            transform: *p_transform,
            ..default()
        };

        sprite_sheet.texture = animation.texture.clone();
        sprite_sheet.atlas = TextureAtlas {
            layout: animation.atlas.clone(),
            index: 1,
        };

        let p1 = p_transform.translation.truncate();
        let p2 = cursor_position.0;
        let px = p2.x - p1.x;
        let py = p2.y - p1.y;
        let angle = f32::atan2(py, px);
        let x = HAMMER_SPEED * f32::cos(angle);
        let y = HAMMER_SPEED * f32::sin(angle);
        let vec = Vec2::new(x, y);

        command.spawn((
            Hammer,
            EntityState::Idle,
            Collider::cuboid(14.0, 14.0),
            Sensor,
            RigidBody::KinematicVelocityBased,
            Velocity::linear(vec),
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
