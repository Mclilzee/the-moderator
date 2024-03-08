mod collision;
use super::physics::collision::collision;
use bevy::prelude::*;

use crate::{
    components::{EntityType, Velocity},
    consts::GRAVITY_ACCELERATION,
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
