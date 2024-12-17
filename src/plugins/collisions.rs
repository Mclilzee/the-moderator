use super::{
    asset_loader::{AnimationEvent, AnimationKey, AnimationMap},
    player::{Player, ScoreUpdateEvent},
};
use crate::common_components::{
    CollisionLayer, Damage, DespawnTimer, Enemy, EntityState, Friendly, Health, Projectile,
};
use avian2d::prelude::{CollisionLayers, CollisionStarted, Collisions, LinearVelocity};
use bevy::prelude::*;

const POINTS_INCREMENT_DURATION: f32 = 1.0;
const POINTS_INCREMENT_ASCENDING_SPEED: f32 = 200.0;
const POINTS_SIZE: f32 = 20.0;
const POINTS_REWARDED: u32 = 1;
const OUT_OF_BOUND_RANGE: f32 = 1000.0;

#[derive(Component)]
struct PointsIncrementEffect;

#[derive(Component)]
struct DeathEffect;

pub struct CollisionsHandlerPlugin;

impl Plugin for CollisionsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (hit, despawn_enemy_on_death)
                .chain()
                .run_if(on_event::<CollisionStarted>),
        )
        .add_systems(
            Update,
            reflect_projectiles.run_if(on_event::<CollisionStarted>),
        )
        .add_systems(Update, despawn_points_increment_effect)
        .add_systems(
            Update,
            animate_death_effect_then_despawn.run_if(on_event::<AnimationEvent>),
        )
        .add_systems(Update, despawn_projectiles_out_of_bound);
    }
}

type WithEnemy = (With<Enemy>, Without<Friendly>);
type WithFriendly = (With<Friendly>, Without<Enemy>);

fn hit(
    mut enemies: Query<(Option<&mut Health>, Option<&Damage>), WithEnemy>,
    mut friendlies: Query<(Option<&mut Health>, Option<&Damage>), WithFriendly>,
    mut collisions: EventReader<CollisionStarted>,
) {
    for CollisionStarted(entity1, entity2) in collisions.read() {
        if let (Ok(friendly), Ok(enemy)) = (friendlies.get_mut(*entity1), enemies.get_mut(*entity2))
        {
            apply_dmg(friendly, enemy);
        } else if let (Ok(friendly), Ok(enemy)) =
            (friendlies.get_mut(*entity2), enemies.get_mut(*entity1))
        {
            apply_dmg(friendly, enemy);
        }
    }
}

fn apply_dmg(
    friendly: (Option<Mut<Health>>, Option<&Damage>),
    enemy: (Option<Mut<Health>>, Option<&Damage>),
) {
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

fn despawn_enemy_on_death(
    mut commands: Commands,
    enemy: Query<(Entity, &Health, &Transform), With<Enemy>>,
    player_transform: Query<&Transform, With<Player>>,
    mut event: EventWriter<ScoreUpdateEvent>,
    asset_loader: Res<AnimationMap>,
) {
    enemy.iter().for_each(|(id, hp, enemy_transform)| {
        if hp.0 <= 0 {
            commands.entity(id).despawn_recursive();
            commands.spawn((
                PointsIncrementEffect,
                DespawnTimer(Timer::from_seconds(
                    POINTS_INCREMENT_DURATION,
                    TimerMode::Once,
                )),
                Text2d::new("++"),
                TextFont::from_font_size(POINTS_SIZE),
                *player_transform.single(),
            ));

            let animation = asset_loader
                .0
                .get(&AnimationKey::DeathEffect)
                .expect("Death effect animation were not found");

            commands.spawn((
                DeathEffect,
                EntityState::Idle,
                Sprite::from_atlas_image(
                    animation.texture.clone(),
                    TextureAtlas {
                        layout: animation.atlas.clone(),
                        index: 1,
                    },
                ),
                *enemy_transform,
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

fn animate_death_effect_then_despawn(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &EntityState), With<DeathEffect>>,
    map: Res<AnimationMap>,
) {
    for (entity, mut sprite, state) in query.iter_mut() {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            let death_effect_animation = map
                .0
                .get(&AnimationKey::DeathEffect)
                .expect("Animation were not found");
            let frames = death_effect_animation
                .indices
                .get(state)
                .unwrap_or(&death_effect_animation.default);

            atlas.index += 1;
            if atlas.index >= frames.last_frame {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn despawn_projectiles_out_of_bound(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform), With<Projectile>>,
    player: Single<&Transform, With<Player>>,
) {
    for (id, transform) in projectiles.iter() {
        if transform.translation.x > player.translation.x + OUT_OF_BOUND_RANGE
            || transform.translation.x < player.translation.x - OUT_OF_BOUND_RANGE
            || transform.translation.y > player.translation.y + OUT_OF_BOUND_RANGE
            || transform.translation.y < player.translation.y - OUT_OF_BOUND_RANGE
        {
            commands.entity(id).despawn();
        }
    }
}

fn reflect_projectiles(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut LinearVelocity, &mut CollisionLayers), With<Projectile>>,
    hitters: Query<Entity, (With<Friendly>, With<Damage>)>,
    collisions: Res<Collisions>,
) {
    for hitter_id in hitters.iter() {
        for (projectile_id, mut velocity, mut layer) in projectiles.iter_mut() {
            if collisions.contains(hitter_id, projectile_id) {
                velocity.0 *= Vec2::NEG_ONE;
                *layer = CollisionLayers::new(
                    CollisionLayer::Friendly,
                    [CollisionLayer::Enemy, CollisionLayer::Wall],
                );

                commands.entity(projectile_id).remove::<Enemy>().insert((
                    Damage(10),
                    Health(1),
                    Friendly,
                ));
            }
        }
    }
}
