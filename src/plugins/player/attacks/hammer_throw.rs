use crate::{
    common_components::{Damage, DespawnTimer, EntityState, Health},
    plugins::{
        asset_loader::{AnimationKey, AnimationMap},
        default_plugins::CursorPosition,
        player::Player,
        spammer_plugin::Spammer,
    },
    AnimationTimer,
};
use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{RigidBody, Velocity},
    geometry::{Collider, CollisionGroups, Group},
    plugin::RapierContext,
};

const HAMMER_SPEED: f32 = 600.0;
const ROATION_ANGLE: f32 = 15.0;
const HEALTH: i32 = 2;
const DAMAGE: i32 = 4;
const DESPAWN_TIMER: f32 = 3.0;

#[derive(Component)]
struct Hammer;

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_button_input)
            .add_systems(Update, collision)
            .add_systems(Update, despawn)
            .add_systems(Update, animate)
            .add_systems(Update, despawn_stopwatch);
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

        let mut sprite_bundle = SpriteBundle {
            transform: *p_transform,
            ..default()
        };

        sprite_bundle.texture = animation.texture.clone();
        let atlas = TextureAtlas {
            layout: animation.atlas.clone(),
            index: 1,
        };

        let p1 = p_transform.translation.truncate();
        let p2 = cursor_position.0;

        command.spawn((
            Hammer,
            Damage(DAMAGE),
            Health(HEALTH),
            DespawnTimer(Timer::from_seconds(DESPAWN_TIMER, TimerMode::Once)),
            EntityState::Idle,
            Collider::cuboid(14.0, 14.0),
            RigidBody::Dynamic,
            CollisionGroups::new(Group::GROUP_1, Group::GROUP_3 | Group::GROUP_2),
            Velocity::linear((p2 - p1).normalize() * HAMMER_SPEED),
            atlas,
            sprite_bundle,
        ));
    }
}

fn animate(
    mut sprite_query: Query<
        (&mut TextureAtlas, &mut Transform, &EntityState, &Velocity),
        With<Hammer>,
    >,
    time: Res<Time>,
    animation_timer: Res<AnimationTimer>,
    animation: Res<AnimationMap>,
) {
    for (mut atlas, mut transform, state, velocity) in sprite_query.iter_mut() {
        let rotation = if velocity.linvel.x > 0.0 { -ROATION_ANGLE } else { ROATION_ANGLE };
        transform.rotate_z(rotation * time.delta_seconds());

        if !animation_timer.0.finished() {
            return;
        }

        let hammer_animation = &animation
            .0
            .get(&AnimationKey::Hammer)
            .expect("Animation were not found");

        let frames = hammer_animation
            .indices
            .get(state)
            .unwrap_or(&hammer_animation.default);

        let mut index = atlas.index + 1;

        if atlas.index >= frames.last_frame || atlas.index < frames.first_frame {
            index = frames.first_frame;
        }

        atlas.index = index;
    }
}

fn collision(
    mut hammers: Query<(Entity, &mut Health, &Damage), (With<Hammer>, Without<Spammer>)>,
    mut spammers: Query<(Entity, &mut Health), (With<Spammer>, Without<Hammer>)>,
    rapier_context: Res<RapierContext>,
) {
    for (h_id, mut h_health, h_dmg) in hammers.iter_mut() {
        for (s_id, mut s_health) in spammers.iter_mut() {
            if rapier_context.contact_pair(h_id, s_id).is_some() {
                s_health.0 -= h_dmg.0;
                h_health.0 = 0;
            }
        }
    }
}

fn despawn(mut commands: Commands, hammers: Query<(Entity, &Health), With<Hammer>>) {
    for (id, health) in hammers.iter() {
        if health.0 <= 0 {
            commands.entity(id).despawn();
        }
    }
}

fn despawn_stopwatch(
    mut commands: Commands,
    mut hammers: Query<(Entity, &mut DespawnTimer), With<Hammer>>,
    time: Res<Time>,
) {
    for (id, mut stopwatch) in hammers.iter_mut() {
        stopwatch.0.tick(time.delta());
        if stopwatch.0.finished() {
            commands.entity(id).despawn();
        }
    }
}
