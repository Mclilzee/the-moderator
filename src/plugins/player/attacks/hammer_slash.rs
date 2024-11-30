use std::time::Duration;

use crate::{
    common_components::{Damage, Enemy, Friendly, Health},
    plugins::{asset_loader::AnimationMap, default_plugins::CursorPosition, player::Player},
};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const SLASH_SPEED: f32 = 600.0;
const WIDTH: f32 = 6.0;
const HEIGHT: f32 = 40.0;
const TRANSFORM_PADDING: f32 = 5.0;
const DAMAGE: i32 = 1;
const COOLDOWN_MILLIS: u64 = 500;
const HAMMER_SWING_RADIUS: f32 = 90.0;

#[derive(Component)]
struct HammerSlash;

#[derive(Resource)]
struct Cooldown(Timer);

pub struct HammerSlashPlugin;

impl Plugin for HammerSlashPlugin {
    fn build(&self, app: &mut App) {
        let mut cooldown = Timer::from_seconds(1.0, TimerMode::Once);
        cooldown.tick(Duration::from_millis(COOLDOWN_MILLIS));

        app.insert_resource(Cooldown(cooldown))
            .add_systems(Update, (despawn, spawn).chain())
            .add_systems(Update, collision);
    }
}

fn spawn(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    cursor_position: Res<CursorPosition>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cooldown: ResMut<Cooldown>,
    time: Res<Time>,
) {
    cooldown.0.tick(time.delta());
    if buttons.just_pressed(MouseButton::Right) && cooldown.0.finished() {
        cooldown.0.reset();
        let (p_id, p_transform) = player_query.single();
        let p1 = p_transform.translation.truncate();
        let p2 = cursor_position.0;

        commands
            .spawn((
                TransformBundle {
                    local: Transform::from_translation(
                        ((p2 - p1).normalize() * Vec2::new(0.0, HEIGHT)).extend(0.0),
                    ),
                    ..default()
                },
                HammerSlash,
                Damage(DAMAGE),
                Collider::cuboid(WIDTH, HEIGHT),
                Friendly,
                Sensor,
                CollisionGroups::new(Group::GROUP_1, Group::GROUP_2 | Group::GROUP_3),
            ))
            .set_parent(p_id);
    }
}

fn despawn(
    mut commands: Commands,
    hammers: Query<Entity, With<HammerSlash>>,
    cooldown: Res<Cooldown>,
) {
    if cooldown.0.finished() {
        hammers.iter().for_each(|id| {
            if let Some(mut entity) = commands.get_entity(id) {
                entity.despawn();
            }
        });
    }
}

fn collision(
    mut hammers: Query<(Entity, &Damage), (With<HammerSlash>, With<Collider>)>,
    mut enemies: Query<(Entity, &mut Health), (Without<HammerSlash>, With<Enemy>, With<Collider>)>,
    rapier_context: Res<RapierContext>,
) {
    for (h_id, h_dmg) in hammers.iter_mut() {
        for (e_id, mut e_hp) in enemies.iter_mut() {
            if rapier_context.intersection_pair(h_id, e_id).is_some() {
                e_hp.0 -= h_dmg.0;
            }
        }
    }
}
