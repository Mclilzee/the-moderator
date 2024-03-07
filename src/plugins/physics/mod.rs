use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{
    components::{Collider, Damage, EntityType, Health, Player, Velocity},
    consts::GRAVITY_ACCELERATION,
    InGameSet,
};

#[derive(Resource)]
struct VelocityPrinter(Timer);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VelocityPrinter(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (movement, print_velocity, collision)
                .chain()
                .in_set(InGameSet::Play),
        );
    }
}

type Colliders<'a> = (
    &'a Collider,
    &'a mut EntityType,
    &'a mut Transform,
    Option<&'a mut Velocity>,
    Option<&'a Damage>,
    Option<&'a mut Health>,
);

fn collision(mut colliders_query: Query<Colliders>) {
    let mut combination = colliders_query.iter_combinations_mut();
    while let Some(
        [(collider1, mut state1, mut transform1, mut velocity1, damage1, mut health1), (collider2, mut state2, mut transform2, mut velocity2, damage2, mut health2)],
    ) = combination.fetch_next()
    {
        let first = Aabb2d::new(
            transform1.translation.truncate(),
            collider1.0 / Vec2::splat(2.0),
        );
        let second = Aabb2d::new(
            transform2.translation.truncate(),
            collider2.0 / Vec2::splat(2.0),
        );

        if !first.intersects(&second) {
            continue;
        }

        solve_solid_collision(
            &first,
            &state1,
            &second,
            &mut transform2.translation,
            velocity2.as_deref_mut(),
            &mut state2,
        );

        solve_solid_collision(
            &second,
            &state2,
            &first,
            &mut transform1.translation,
            velocity1.as_deref_mut(),
            &mut state1,
        );

        solve_damage(
            damage1,
            health1.as_deref_mut(),
            damage2,
            health2.as_deref_mut(),
        );
    }
}

fn solve_damage(
    dmg1: Option<&Damage>,
    health1: Option<&mut Health>,
    dmg2: Option<&Damage>,
    health2: Option<&mut Health>,
) {
    if let Some(dmg) = dmg1 {
        if let Some(hp) = health2 {
            hp.0 -= dmg.0;
        }
    }

    if let Some(dmg) = dmg2 {
        if let Some(hp) = health1 {
            hp.0 -= dmg.0;
        }
    }
}

fn solve_solid_collision(
    solid_boundary: &Aabb2d,
    solid_state: &EntityType,
    entity_boundary: &Aabb2d,
    entity_translation: &mut Vec3,
    entity_velocity: Option<&mut Velocity>,
    entity_state: &mut EntityType,
) {
    if *solid_state != EntityType::Solid
        || (*entity_state == EntityType::Solid && entity_velocity.is_none())
    {
        return;
    }

    match find_collision_side(solid_boundary, entity_boundary) {
        CollisionSide::Left => {
            entity_translation.x = solid_boundary.min.x - (entity_boundary.half_size().x);
        }
        CollisionSide::Right => {
            entity_translation.x = solid_boundary.max.x + (entity_boundary.half_size().x);
        }
        CollisionSide::Top => {
            entity_translation.y = solid_boundary.max.y + (entity_boundary.half_size().y);
            if let Some(velocity) = entity_velocity {
                velocity.0.y = 0.0;
            }
        }
        CollisionSide::Bottom => {
            entity_translation.y = solid_boundary.min.y - (entity_boundary.half_size().y);
            if let Some(velocity) = entity_velocity {
                velocity.0.y = 0.0;
            }
        }
    }
}

enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
}

fn find_collision_side(solid: &Aabb2d, entity: &Aabb2d) -> CollisionSide {
    let center = entity.center();
    let closest = solid.closest_point(center);
    let offset = center - closest;
    let abs = offset.abs() - entity.half_size();

    if solid.contains(entity) || abs.y > abs.x {
        if offset.y < 0.0 {
            CollisionSide::Bottom
        } else {
            CollisionSide::Top
        }
    } else if offset.x < 0.0 {
        CollisionSide::Left
    } else {
        CollisionSide::Right
    }
}

type MovingActors<'a> = (&'a mut Transform, &'a mut Velocity, &'a EntityType);
fn movement(mut actors_query: Query<MovingActors>, time: Res<Time>) {
    for (mut transform, mut velocity, entity_type) in actors_query.iter_mut() {
        let delta_time = time.delta_seconds();
        transform.translation += velocity.0.extend(0.0) * delta_time;

        if let EntityType::Grounded = *entity_type {
            velocity.0.y -= GRAVITY_ACCELERATION * delta_time;
            if velocity.0.y < -GRAVITY_ACCELERATION {
                velocity.0.y = -GRAVITY_ACCELERATION;
            }
        }
    }
}

fn print_velocity(
    velocity_query: Query<&Velocity, With<Player>>,
    time: Res<Time>,
    mut velocity_timer: ResMut<VelocityPrinter>,
) {
    if velocity_timer.0.tick(time.delta()).finished() {
        let velocity = velocity_query.single();
        info!("Velocity {}", velocity.0.y);
    }
}
