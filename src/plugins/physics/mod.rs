mod collider;
pub mod state;
use self::collider::{CollidePosition, PlatformCollider};
use bevy::prelude::*;

use crate::{
    components::{Collider, Jumps, Platform, Velocity},
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
    &'a mut Transform,
    Option<&'a mut Velocity>,
    Option<&'a mut Jumps>,
    EntityState,
);

fn collision(mut colliders_query: Query<Colliders>) {
    let (platform_transform, platform_sprite) = platform_query.single();
    let platform_size = match platform_sprite.custom_size {
        Some(vec) => vec,
        None => return,
    };

    for (collider, mut transform, maybe_velocity, maybe_jump, state) in
        colliders_query.mut_iter_combinations()
    {
        let collider = PlatformCollider::new(&platform_transform.translation, &platform_size);

        for (boundary_box, mut transform, mut velocity, jumps) in actors_query.iter_mut() {
            let position = collider.position(&transform.translation, &boundary_box.size);
            match position {
                CollidePosition::Top(position) => {
                    transform.translation = position;
                    velocity.0.y = 0.0;
                    if let Some(mut jumps) = jumps {
                        jumps.current = jumps.max;
                    }
                }
                CollidePosition::Bottom(position) => {
                    transform.translation = position;
                    velocity.0.y = 0.0;
                }
                CollidePosition::Left(position) => transform.translation = position,
                CollidePosition::Right(position) => transform.translation = position,
                CollidePosition::None => (),
            }
        }
    }
}

type MovingActors<'a> = (&'a mut Transform, &'a mut Velocity);

fn movement(mut actors_query: Query<MovingActors>, time: Res<Time>) {
    for (mut transform, mut velocity) in actors_query.iter_mut() {
        velocity.0.y -= GRAVITY_ACCELERATION;
        if velocity.0.y == GRAVITY_MAX_SPEED {
            velocity.0.y = GRAVITY_MAX_SPEED;
        }

        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    }
}
