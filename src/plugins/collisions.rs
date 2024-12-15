use super::player::ScoreUpdateEvent;
use crate::common_components::{Damage, DespawnTimer, Enemy, Friendly, Health};
use avian2d::prelude::CollisionStarted;
use bevy::prelude::*;

const POINTS_INCREMENT_DURATION: f32 = 1.0;
const POINTS_INCREMENT_ASCENDING_SPEED: f32 = 200.0;
const POINTS_SIZE: f32 = 20.0;
const POINTS_REWARDED: u32 = 1;

#[derive(Component)]
struct PointsIncrementEffect;

pub struct CollisionsHandlerPlugin;

impl Plugin for CollisionsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (hit, despawn_enemy_on_death)
                .chain()
                .run_if(on_event::<CollisionStarted>),
        )
        .add_systems(Update, despawn_points_increment_effect);
    }
}

fn hit(
    mut enemies: Query<(Option<&mut Health>, Option<&Damage>), (With<Enemy>, Without<Friendly>)>,
    mut allies: Query<(Option<&mut Health>, Option<&Damage>), (With<Friendly>, Without<Enemy>)>,
    mut collisions: EventReader<CollisionStarted>,
) {
    for CollisionStarted(entity1, entity2) in collisions.read() {
        if let (Ok(friendly), Ok(enemy)) = (allies.get_mut(*entity1), enemies.get_mut(*entity2)) {
            if let Some(mut hp) = friendly.0 {
                if let Some(dmg) = enemy.1 {
                    hp.0 -= dmg.0;
                }
            }

            if let Some(mut hp) = enemy.0 {
                if let Some(dmg) = friendly.1 {
                    hp.0 -= dmg.0;
                }
            }
        } else if let (Ok(friendly), Ok(enemy)) =
            (allies.get_mut(*entity2), enemies.get_mut(*entity1))
        {
            if let Some(mut hp) = friendly.0 {
                if let Some(dmg) = enemy.1 {
                    hp.0 -= dmg.0;
                }
            }

            if let Some(mut hp) = enemy.0 {
                if let Some(dmg) = friendly.1 {
                    hp.0 -= dmg.0;
                }
            }
        }
    }
}

fn despawn_enemy_on_death(
    mut commands: Commands,
    query: Query<(Entity, &Health, &Transform), With<Enemy>>,
    mut event: EventWriter<ScoreUpdateEvent>,
) {
    query.iter().for_each(|(id, hp, transform)| {
        if hp.0 <= 0 {
            commands.entity(id).despawn();
            commands.spawn((
                PointsIncrementEffect,
                DespawnTimer(Timer::from_seconds(
                    POINTS_INCREMENT_DURATION,
                    TimerMode::Once,
                )),
                Text2d::new("++"),
                TextFont::from_font_size(POINTS_SIZE),
                *transform,
            ));

            event.send(ScoreUpdateEvent {
                gained_points: POINTS_REWARDED,
            });
        }
    });
}

fn despawn_points_increment_effect(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnTimer, &mut Transform), With<PointsIncrementEffect>>,
    time: Res<Time>,
) {
    for (id, mut timer, mut transform) in query.iter_mut() {
        timer.0.tick(time.delta());
        transform.translation.y += POINTS_INCREMENT_ASCENDING_SPEED * time.delta_secs();
        if timer.0.finished() {
            commands.entity(id).despawn();
        }
    }
}
