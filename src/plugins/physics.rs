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

fn collision(
    mut entities_query: Query<
        (&HitBox, &mut Transform, &mut Velocity, Option<&mut Jumps>),
        (With<HitBox>, Without<Platform>),
    >,
    platform_query: Query<(&Transform, &Sprite), With<Platform>>,
) {
    let (platform_transform, platform_sprite) = platform_query.single();
    let platform_size = match platform_sprite.custom_size {
        Some(vec) => vec,
        None => return,
    };

    let platform_y = platform_transform.translation.y + (platform_size.y / 2.0);

    for (hitbox, mut transform, mut velocity, jumps) in entities_query.iter_mut() {
        let height = hitbox.0.y / 2.0;
        if transform.translation.y - height < platform_y {
            transform.translation.y = platform_y + height;
            velocity.translation.y = 0.0;
            if let Some(mut jumps) = jumps {
                jumps.0 = 2;
            }
        }
    }
}
