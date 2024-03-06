use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{
    components::{Collider, EntityState, Velocity},
    consts::{GRAVITY_ACCELERATION, GRAVITY_MAX_SPEED},
    InGameSet,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (movement, collision).chain().in_set(InGameSet::Play),
        );
    }
}

type Colliders<'a> = (
    &'a Collider,
    &'a mut EntityState,
    &'a mut Transform,
    Option<&'a mut Velocity>,
);

fn collision(mut colliders_query: Query<Colliders>) {
    let mut combination = colliders_query.iter_combinations_mut();
    while let Some(
        [(collider1, mut state1, mut transform1, mut velocity1), (collider2, mut state2, mut transform2, mut velocity2)],
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
            &velocity2,
            &mut state2,
        );

        solve_solid_collision(
            &second,
            &state2,
            &first,
            &mut transform1.translation,
            &velocity1,
            &mut state1,
        );
    }
}

fn solve_solid_collision(
    solid_boundary: &Aabb2d,
    solid_state: &EntityState,
    entity_boundary: &Aabb2d,
    entity_translation: &mut Vec3,
    entity_velocity: &Option<Mut<'_, Velocity>>,
    entity_state: &mut EntityState,
) {
    if *solid_state != EntityState::Solid
        || (*entity_state == EntityState::Solid && entity_velocity.is_none())
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
            // if let Some(&mut velocity) = *entity_velocity {
            //     velocity.0.y = 0.0;
            // }
            *entity_state = EntityState::Grounded;
        }
        CollisionSide::Bottom => {
            info!("Bottom");
            entity_translation.y = solid_boundary.min.y - (entity_boundary.half_size().y);
            // if let Some(&mut velocity) = *entity_velocity {
            //     velocity.0.y = 0.0;
            // }
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

type MovingActors<'a> = (&'a mut Transform, &'a mut Velocity);

fn movement(mut actors_query: Query<MovingActors>, time: Res<Time>) {
    for (mut transform, mut velocity) in actors_query.iter_mut() {
        velocity.0.y -= GRAVITY_ACCELERATION;
        if velocity.0.y < -GRAVITY_MAX_SPEED {
            velocity.0.y = -GRAVITY_MAX_SPEED;
        }

        let velocity = velocity.0.extend(0.0) * time.delta_seconds();
        transform.translation += velocity;
    }
}
