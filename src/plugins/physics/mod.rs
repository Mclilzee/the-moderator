use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{
    components::{Collider, Damage, EntityType, Health, Player, Velocity},
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
            set_grounded(&mut state1, velocity1.as_deref_mut());
            set_grounded(&mut state2, velocity2.as_deref_mut());
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

fn set_grounded(state: &mut EntityType, velocity: Option<&mut Velocity>) {
    if *state == EntityType::Grounded {
        if let Some(velocity) = &velocity {
            if velocity.0.y < 0.0 {
                *state = EntityType::Falling;
            } else {
                *state = EntityType::Jumping;
            }
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
            *entity_state = EntityType::Grounded;
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
fn movement(mut actors_query: Query<MovingActors, With<Player>>, time: Res<Time>) {
    for (mut transform, mut velocity, state) in actors_query.iter_mut() {
        match *state {
            EntityType::Jumping | EntityType::Falling => {
                velocity.0.y -= GRAVITY_ACCELERATION;
                if velocity.0.y < -GRAVITY_MAX_SPEED {
                    velocity.0.y = -GRAVITY_MAX_SPEED;
                }
            }
            _ => (),
        }

        // info!("State {:?}", state);

        let velocity = velocity.0.extend(0.0) * time.delta_seconds();
        // println!("Velocity {}", velocity.y);
        transform.translation += velocity;
    }
}
