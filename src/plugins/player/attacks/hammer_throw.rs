use std::time::Duration;

use crate::{
    common_components::{
        CollisionLayer, Damage, DespawnTimer, EntityState, Friendly, Health,
    },
    plugins::{
        asset_loader::{AnimationEvent, AnimationKey, AnimationMap},
        default_plugins::CursorPosition,
        player::Player,
    },
    utils::animate,
};

use avian2d::prelude::AngularVelocity;
use avian2d::prelude::Collider;
use avian2d::prelude::{
    ColliderDensity, CollisionLayers, LinearVelocity, Restitution, RigidBody,
};
use bevy::prelude::*;

const HAMMER_DENSITY: f32 = 20.0;
const HAMMER_SPEED: f32 = 600.0;
const ROATION_ANGLE: f32 = 10.0;
const HEALTH: i32 = 5;
const DAMAGE: i32 = 5;
const DESPAWN_TIMER: f32 = 5.0;
const COOLDOWN_SECS: f32 = 0.5;
const HAMMER_SHAPE: (Vec2, Vec2, Vec2) = (
    Vec2::new(-15.0, -15.0),
    Vec2::new(18.0, 0.0),
    Vec2::new(0.0, 18.0),
);

#[derive(Component)]
struct HammerThrow;

#[derive(Resource)]
struct Cooldown(Timer);

pub struct HammerThrowPlugin;

impl Plugin for HammerThrowPlugin {
    fn build(&self, app: &mut App) {
        let mut cooldown = Timer::from_seconds(COOLDOWN_SECS, TimerMode::Once);
        cooldown.tick(Duration::from_secs_f32(COOLDOWN_SECS));

        app.insert_resource(Cooldown(cooldown))
            .add_systems(Update, animate_hammer.run_if(on_event::<AnimationEvent>))
            .add_systems(Update, cooldown_tick)
            .add_systems(
                Update,
                mouse_button_input.run_if(resource_changed::<ButtonInput<MouseButton>>),
            )
            .add_systems(Update, despawn)
            .add_systems(Update, despawn_timer);
    }
}

fn cooldown_tick(time: Res<Time>, mut cooldown_timer: ResMut<Cooldown>) {
    cooldown_timer.0.tick(time.delta());
}

fn mouse_button_input(
    mut command: Commands,
    player_query: Query<&Transform, With<Player>>,
    cursor_position: Res<CursorPosition>,
    buttons: Res<ButtonInput<MouseButton>>,
    animation_map: Res<AnimationMap>,
    mut cooldown: ResMut<Cooldown>,
) {
    if buttons.pressed(MouseButton::Left) && cooldown.0.finished() {
        cooldown.0.reset();
        let animation = animation_map
            .0
            .get(&AnimationKey::HammerThrow)
            .expect("Hammer Image animation were not found");

        let p_transform = player_query.single();
        let p1 = p_transform.translation.truncate();
        let p2 = cursor_position.0;

        let l_velocity = LinearVelocity::from((p2 - p1).normalize() * HAMMER_SPEED);
        let a_velocity = AngularVelocity::from(if l_velocity.x >= 0.0 {
            -ROATION_ANGLE
        } else {
            ROATION_ANGLE
        });

        command.spawn((
            HammerThrow,
            Sprite::from_atlas_image(
                animation.texture.clone(),
                TextureAtlas {
                    layout: animation.atlas.clone(),
                    index: 1,
                },
            ),
            Damage(DAMAGE),
            Health(HEALTH),
            Friendly,
            DespawnTimer(Timer::from_seconds(DESPAWN_TIMER, TimerMode::Once)),
            EntityState::Idle,
            RigidBody::Dynamic,
            Collider::triangle(HAMMER_SHAPE.0, HAMMER_SHAPE.1, HAMMER_SHAPE.2),
            Restitution::PERFECTLY_INELASTIC,
            ColliderDensity(HAMMER_DENSITY),
            CollisionLayers::new(
                CollisionLayer::Friendly,
                [CollisionLayer::Enemy, CollisionLayer::Wall],
            ),
            *p_transform,
            l_velocity,
            a_velocity,
        ));
    }
}

fn despawn(
    mut commands: Commands,
    hammers: Query<(Entity, &Health, &LinearVelocity), With<HammerThrow>>,
) {
    for (id, health, velocity) in hammers.iter() {
        if health.0 <= 0 || velocity.0 == Vec2::ZERO {
            commands.entity(id).despawn();
        }
    }
}

fn despawn_timer(
    mut commands: Commands,
    mut hammers: Query<(Entity, &mut DespawnTimer), With<HammerThrow>>,
    time: Res<Time>,
) {
    for (id, mut timer) in hammers.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(id).despawn();
        }
    }
}

fn animate_hammer(
    mut query: Query<(&mut Sprite, &EntityState), With<HammerThrow>>,
    map: Res<AnimationMap>,
) {
    query.iter_mut().for_each(|(mut sprite, state)| {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            animate(atlas, state, &AnimationKey::HammerThrow, &map);
        }
    });
}
