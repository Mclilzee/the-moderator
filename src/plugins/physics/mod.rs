mod collider;
use self::collider::{CollidePosition, PlatformCollider};
use bevy::prelude::*;

use crate::{
    components::{HitBox, Jumps, Platform, Velocity},
    InGameSet,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision.in_set(InGameSet::Play));
    }
}

type Entities<'a> = (
    &'a HitBox,
    &'a mut Transform,
    &'a mut Velocity,
    Option<&'a mut Jumps>,
);

fn collision(
    mut entities_query: Query<Entities, Without<Platform>>,
    platform_query: Query<(&Transform, &Sprite), With<Platform>>,
) {
    let (platform_transform, platform_sprite) = platform_query.single();
    let platform_size = match platform_sprite.custom_size {
        Some(vec) => vec,
        None => return,
    };

    let collider = PlatformCollider::new(&platform_transform.translation, &platform_size);

    for (hitbox, mut transform, mut velocity, jumps) in entities_query.iter_mut() {
        let position = collider.position(&transform.translation, &hitbox.0);
        match position {
            CollidePosition::Top(position) => {
                transform.translation = position;
                velocity.translation.y = 0.0;
                if let Some(mut jumps) = jumps {
                    jumps.current = jumps.max;
                }
            }
            CollidePosition::Bottom(position) => {
                transform.translation = position;
                velocity.translation.y = 0.0;
            }
            CollidePosition::Left(position) => transform.translation = position,
            CollidePosition::Right(position) => transform.translation = position,
            CollidePosition::None => return,
        }
    }
}
