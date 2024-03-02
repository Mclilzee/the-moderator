use bevy::prelude::*;

use crate::{
    components::{HitBox, Jumps, Platform, Player, Velocity},
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
    mut entities_query: Query<Entities, (Without<Platform>, With<Player>)>,
    platform_query: Query<(&Transform, &Sprite), With<Platform>>,
) {
    let (platform_transform, platform_sprite) = platform_query.single();
    let platform_size = match platform_sprite.custom_size {
        Some(vec) => vec,
        None => return,
    };

    let platform_left = platform_transform.translation.x - (platform_size.x / 2.0);
    let platform_right = platform_transform.translation.x + (platform_size.x / 2.0);
    let platform_top = platform_transform.translation.y + (platform_size.y / 2.0);
    let platform_bottom = platform_transform.translation.y - (platform_size.y / 2.0);

    for (hitbox, mut transform, mut velocity, jumps) in entities_query.iter_mut() {
        let height = hitbox.0.y / 2.0;
        let width = hitbox.0.x / 2.0;
        let entity_left = transform.translation.x - (hitbox.0.x / 2.0);
        let entity_right = transform.translation.x + (hitbox.0.x / 2.0);
        let entity_top = transform.translation.y + (hitbox.0.y / 2.0);
        let entity_bottom = transform.translation.y - (hitbox.0.y / 2.0);

        info!("Player: l {}, r {}", entity_left, entity_right);
        info!("Platform: l {}, r {}", platform_left, platform_right);

        if (entity_right > platform_left && entity_left < platform_right)
            && entity_bottom < platform_top
        {
            transform.translation.y = platform_top + height;
            velocity.translation.y = 0.0;
            if let Some(mut jumps) = jumps {
                jumps.current = jumps.max;
            }
        }
    }
}
