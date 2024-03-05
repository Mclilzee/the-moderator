use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{
    components::{Collider, EntityState, Spammer, Velocity},
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

fn collision(mut colliders_query: Query<Colliders, Without<Spammer>>) {
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

        if *state1 == EntityState::Solid && (*state2 != EntityState::Solid || velocity2.is_some()) {
            match find_collision_side(&first, &second) {
                CollisionSide::Left => {
                    transform2.translation.x = transform1.translation.x - (collider2.0.x / 2.0);
                }
                CollisionSide::Right => {
                    transform2.translation.x = transform1.translation.x + (collider2.0.x / 2.0);
                }
                CollisionSide::Top => {
                    transform2.translation.y = transform1.translation.y + (collider2.0.y / 2.0);
                    if let Some(mut velocity) = velocity2 {
                        velocity.0.y = 0.0;
                    }
                    *state2 = EntityState::Grounded;
                }
                CollisionSide::Bottom => {
                    transform2.translation.y = transform1.translation.y - (collider2.0.y / 2.0);
                    if let Some(mut velocity) = velocity2 {
                        velocity.0.y = 0.0;
                    }
                }
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

fn find_collision_side(first: &Aabb2d, second: &Aabb2d) -> CollisionSide {
    let closest = first.closest_point(second.center());
    let offset = second.center() - closest;
    let abs = offset.abs() - second.half_size();
    println!("Offset Abs: {abs}");

    if first.contains(second) || abs.y > abs.x {
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
        if velocity.0.y > GRAVITY_MAX_SPEED {
            velocity.0.y = -GRAVITY_MAX_SPEED;
        }

        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    }
}
