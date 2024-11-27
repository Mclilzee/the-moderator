use std::time::Duration;

use crate::{
    common_components::{Damage, Health},
    plugins::{asset_loader::AnimationMap, default_plugins::CursorPosition, player::Player},
};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const SLASH_SPEED: f32 = 600.0;
const DAMAGE: i32 = 1;
const COOLDOWN_SECS: u64 = 1;

#[derive(Component)]
struct HammerSlash;

#[derive(Resource)]
struct Cooldown(Timer);

pub struct HammerSlashPlugin;

impl Plugin for HammerSlashPlugin {
    fn build(&self, app: &mut App) {
        let mut cooldown = Timer::from_seconds(1.0, TimerMode::Once);
        cooldown.tick(Duration::from_secs(COOLDOWN_SECS));

        app.insert_resource(Cooldown(cooldown))
            .add_systems(Update, (cooldown_tick, swing).chain())
            .add_systems(Update, collision);
    }
}

fn cooldown_tick(time: Res<Time>, mut cooldown_timer: ResMut<Cooldown>) {
    cooldown_timer.0.tick(time.delta());
}

fn swing(
    mut command: Commands,
    player: Query<&Transform, With<Player>>,
    cursor_position: Res<CursorPosition>,
    buttons: Res<ButtonInput<MouseButton>>,
    animation_map: Res<AnimationMap>,
    mut cooldown: ResMut<Cooldown>,
) {
    if buttons.just_pressed(MouseButton::Right) && cooldown.0.finished() {
        cooldown.0.reset();

        let p_transform = player.single();

        let sprite_bundle = SpriteBundle {
            transform: *p_transform,
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 6.0)),
                ..default()
            },
            ..default()
        };

        let p1 = p_transform.translation.truncate();
        let p2 = cursor_position.0;

        let mut velocity = Velocity::linear((p2 - p1).normalize() * SLASH_SPEED);

        command.spawn((
            HammerSlash,
            Damage(DAMAGE),
            Collider::cuboid(20.0, 6.0),
            Sensor,
            Restitution::coefficient(0.0),
            CollisionGroups::new(Group::GROUP_1, Group::GROUP_3 | Group::GROUP_2),
            sprite_bundle,
        ));
    }
}

fn collision(
    mut hammers: Query<(Entity, &Damage), (With<HammerSlash>, With<Collider>)>,
    mut enemies: Query<
        (Entity, &mut Health),
        (Without<HammerSlash>, Without<Player>, With<Collider>),
    >,
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
