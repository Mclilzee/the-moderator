use std::time::Duration;

use crate::{
    common_components::{Damage, DespawnTimer, EntityState, Health},
    plugins::{
        asset_loader::{AnimationKey, AnimationMap},
        default_plugins::CursorPosition,
        player::Player,
    },
};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const HAMMER_SPEED: f32 = 600.0;
const ROATION_ANGLE: f32 = 10.0;
const HEALTH: i32 = 10;
const DAMAGE: i32 = 4;
const DESPAWN_TIMER: f32 = 30.0;
const COOLDOWN_SECS: u64 = 1;

#[derive(Component)]
struct Hammer;

#[derive(Resource)]
struct Cooldown(Timer);

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        let mut cooldown = Timer::from_seconds(1.0, TimerMode::Once);
        cooldown.tick(Duration::from_secs(COOLDOWN_SECS));

        app.insert_resource(Cooldown(cooldown))
            .add_systems(Update, (cooldown_tick, mouse_button_input).chain())
            .add_systems(Update, collision)
            .add_systems(Update, despawn)
            .add_systems(Update, despawn_timer);
    }
}

fn cooldown_tick(time: Res<Time>, mut cooldown_timer: ResMut<Cooldown>) {
    cooldown_timer.0.tick(time.delta());
}

fn mouse_button_input(
    mut command: Commands,
    player: Query<&Transform, With<Player>>,
    cursor_position: Res<CursorPosition>,
    buttons: Res<ButtonInput<MouseButton>>,
    animation_map: Res<AnimationMap>,
    mut cooldown: ResMut<Cooldown>,
) {
    if buttons.just_pressed(MouseButton::Left) && cooldown.0.finished() {
        cooldown.0.reset();
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

        let mut velocity = Velocity::linear((p2 - p1).normalize() * HAMMER_SPEED);
        velocity.angvel = if velocity.linvel.x >= 0.0 {
            -ROATION_ANGLE
        } else {
            ROATION_ANGLE
        };

        command.spawn((
            Hammer,
            Damage(DAMAGE),
            Health(HEALTH),
            AnimationKey::Hammer,
            DespawnTimer(Timer::from_seconds(DESPAWN_TIMER, TimerMode::Once)),
            EntityState::Idle,
            Collider::cuboid(14.0, 14.0),
            Restitution::coefficient(0.0),
            RigidBody::Dynamic,
            CollisionGroups::new(Group::GROUP_1, Group::GROUP_3 | Group::GROUP_2),
            velocity,
            atlas,
            sprite_bundle,
        ));
    }
}

fn collision(
    mut hammers: Query<(Entity, &mut Health, &Damage), (With<Hammer>, With<Collider>)>,
    mut enemies: Query<
        (Entity, &mut Health, &Damage),
        (Without<Hammer>, Without<Player>, With<Collider>),
    >,
    rapier_context: Res<RapierContext>,
) {
    for (h_id, mut h_hp, h_dmg) in hammers.iter_mut() {
        for (e_id, mut e_hp, e_dmg) in enemies.iter_mut() {
            if rapier_context.contact_pair(h_id, e_id).is_some() {
                e_hp.0 -= h_dmg.0;
                h_hp.0 -= e_dmg.0;
            }
        }
    }
}

fn despawn(mut commands: Commands, hammers: Query<(Entity, &Health, &Velocity), With<Hammer>>) {
    for (id, health, velocity) in hammers.iter() {
        if health.0 <= 0 || velocity.linvel == Vec2::ZERO {
            commands.entity(id).despawn();
        }
    }
}

fn despawn_timer(
    mut commands: Commands,
    mut hammers: Query<(Entity, &mut DespawnTimer), With<Hammer>>,
    time: Res<Time>,
) {
    for (id, mut timer) in hammers.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(id).despawn();
        }
    }
}
