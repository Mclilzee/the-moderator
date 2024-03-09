use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::components::{Collider, Damage, Grounded, Health, Solid, Velocity};

enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
}

type Colliders<'a> = (
    &'a Collider,
    &'a mut Transform,
    Option<&'a Solid>,
    Option<&'a mut Velocity>,
    Option<&'a mut Grounded>,
    Option<&'a Damage>,
    Option<&'a mut Health>,
);

pub fn collision(mut colliders_query: Query<Colliders>) {
    let mut combination = colliders_query.iter_combinations_mut();
    while let Some(
        [(collider1, mut transform1, solid1, mut velocity1, mut grounded1, damage1, mut health1), (collider2, mut transform2, solid2, mut velocity2, mut grounded2, damage2, mut health2)],
    ) = combination.fetch_next()
    {
        let first_collider = Aabb2d::new(
            transform1.translation.truncate(),
            collider1.0 / Vec2::splat(2.0),
        );
        let second_collider = Aabb2d::new(
            transform2.translation.truncate(),
            collider2.0 / Vec2::splat(2.0),
        );

        if !first_collider.intersects(&second_collider) {
            if solid1.is_some() && grounded2.is_some() {
                grounded2.unwrap().0 = false;
            }

            if solid2.is_some() && grounded1.is_some() {
                grounded1.unwrap().0 = false;
            }
            continue;
        }

        if solid1.is_some() {
            solve_platform_collision(
                &first_collider,
                &solid2,
                &second_collider,
                &mut transform2.translation,
                velocity2.as_deref_mut(),
                grounded2.as_deref_mut(),
            );
        }

        if solid2.is_some() {
            solve_platform_collision(
                &second_collider,
                &solid1,
                &first_collider,
                &mut transform1.translation,
                velocity1.as_deref_mut(),
                grounded1.as_deref_mut(),
            );
        }

        solve_damage(
            damage1,
            health1.as_deref_mut(),
            damage2,
            health2.as_deref_mut(),
        );
    }
}

fn solve_platform_collision(
    solid_boundary: &Aabb2d,
    entity_platform: &Option<&Solid>,
    entity_boundary: &Aabb2d,
    entity_translation: &mut Vec3,
    entity_velocity: Option<&mut Velocity>,
    entity_grounded: Option<&mut Grounded>,
) {
    if entity_platform.is_some() && entity_velocity.is_none() {
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
            if let Some(grounded) = entity_grounded {
                grounded.0 = true;
            }
        }
        CollisionSide::Bottom => {
            entity_translation.y = solid_boundary.min.y - (entity_boundary.half_size().y);
        }
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
