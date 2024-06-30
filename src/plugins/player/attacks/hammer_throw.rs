use std::ops::Neg;

use crate::{
    components::{EntityState, Player, Spammer},
    plugins::{
        asset_loader::{AnimationKey, AnimationMap},
        default_plugins::CursorPosition,
    },
    AnimationTimer,
};
use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{RigidBody, Velocity},
    geometry::{ActiveEvents, Collider, CollisionGroups, Group, Sensor},
    plugin::RapierContext,
};

const HAMMER_SPEED: f32 = 350.0;
const ROTATION_SPEED: f32 = 2.0;

#[derive(Component)]
struct Hammer;

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_button_input)
            .add_systems(Update, collision)
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

        command.spawn((
            Hammer,
            EntityState::Idle,
            Collider::cuboid(14.0, 14.0),
            RigidBody::KinematicVelocityBased,
            CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
            ActiveEvents::COLLISION_EVENTS,
            Velocity::linear((p2 - p1).normalize() * HAMMER_SPEED),
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

fn collision(
    mut commands: Commands,
    mut hammers: Query<(Entity, &mut Velocity), (With<Hammer>, Without<Spammer>)>,
    spammers: Query<Entity, (With<Spammer>, Without<Hammer>)>,
    rapier_context: Res<RapierContext>,
) {
    for (hammer, mut vel) in hammers.iter_mut() {
        for spammer in spammers.iter() {
            if rapier_context.intersection_pair(hammer, spammer) == Some(true) {
                commands.entity(spammer).despawn();
                vel.linvel = vel.linvel.neg();
            }
        }
    }
}
