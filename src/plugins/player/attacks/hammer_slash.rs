use std::time::Duration;

use crate::{
    common_components::{Damage, Enemy, Friendly, Health},
    plugins::player::{Player, PLAYER_HEIGHT},
};

use avian2d::prelude::{Collider, Sensor};
use bevy::prelude::*;

const SMASH_WIDTH: f32 = 60.0;
const SMASH_HEIGHT: f32 = 1.0;
const DAMAGE: i32 = 10;
const COOLDOWN_MILLIS: u64 = 500;

#[derive(Component)]
struct GroundSmash;

#[derive(Resource, Deref, DerefMut)]
struct Cooldown(Timer);

#[derive(Resource, Deref, DerefMut)]
struct DespawnTimer(Timer);

pub struct GroundSmashPlugin;

impl Plugin for GroundSmashPlugin {
    fn build(&self, app: &mut App) {
        let mut cooldown = Timer::from_seconds(1.0, TimerMode::Once);
        cooldown.tick(Duration::from_millis(COOLDOWN_MILLIS));

        app.insert_resource(Cooldown(cooldown)).insert_resource(DespawnTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
            .add_systems(Update, (spawn, collision).chain())
            .add_systems(Update, despawn);
    }
}

fn spawn(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cooldown: ResMut<Cooldown>,
    time: Res<Time>,
) {
    cooldown.0.tick(time.delta());
    if buttons.just_pressed(MouseButton::Right) && cooldown.0.finished() {
        cooldown.0.reset();
        commands
            .spawn((
                TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0 - PLAYER_HEIGHT, 0.0)),
                GroundSmash,
                Damage(DAMAGE),
                Collider::capsule(SMASH_WIDTH, SMASH_HEIGHT),
                Friendly,
                Sensor,
            ))
            .set_parent(player.single());
    }
}

fn despawn(
    mut commands: Commands,
    hammers: Query<Entity, With<GroundSmash>>,
    mut despawn_timer: ResMut<DespawnTimer>,
    time: Res<Time>,
) {
    despawn_timer.tick(time.delta());
    if despawn_timer.finished() {
        hammers.iter().for_each(|id| {
            if let Some(mut entity) = commands.get_entity(id) {
                entity.despawn();
            }
        });
    }
}

// TODO: add animation and despawn after animation finish

fn collision(
    mut hammers: Query<(Entity, &Damage), (With<GroundSmash>, With<Collider>)>,
    mut enemies: Query<(Entity, &mut Health), (Without<GroundSmash>, With<Enemy>, With<Collider>)>,
) {
    //for (h_id, h_dmg) in hammers.iter_mut() {
    //    for (e_id, mut e_hp) in enemies.iter_mut() {
    //        if rapier_context.intersection_pair(h_id, e_id).is_some() {
    //            e_hp.0 -= h_dmg.0;
    //        }
    //    }
    //}
}
