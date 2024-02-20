use bevy::prelude::*;

use crate::{components::Velocity, InGameSet};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_movement.in_set(InGameSet::Movement));
    }
}

fn update_movement(mut query: Query<(&mut Transform, &Velocity)>) {
    query
        .iter_mut()
        .for_each(|(mut transform, velocity)| transform.translation += velocity.translation);
}
