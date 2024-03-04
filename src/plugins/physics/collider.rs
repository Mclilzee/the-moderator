use bevy::{
    log::info,
    math::{
        bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
        Vec2, Vec3,
    },
};

pub enum CollidePosition {
    Top(Vec3),
    Bottom(Vec3),
    Left(Vec3),
    Right(Vec3),
    None,
}

pub struct PlatformCollider {
    platform: Aabb2d,
}

impl PlatformCollider {
    pub fn new(translation: &Vec3, size: &Vec2) -> Self {
        PlatformCollider {
            platform: Aabb2d::new(translation.truncate(), *size / Vec2::new(2.0, 2.0)),
        }
    }

    pub fn position(&self, translation: &Vec3, size: &Vec2) -> CollidePosition {
        let entity = Aabb2d::new(translation.truncate(), *size / Vec2::new(2.0, 2.0));
        if !entity.intersects(&self.platform) {
            return CollidePosition::None;
        }

        let height = size.y / 2.0;
        let width = size.x / 2.0;
        let closest = self.platform.closest_point(entity.center());
        let offset = entity.center() - closest;

        println!("Closest: {closest}, Offset: {offset}");
        if offset.y == 0.0 {
            if offset.x < 0.0 {
                info!("Colliding Left");
            } else {
                info!("Colliding Right");
            }
        } else if offset.y > 0.0 {
            info!("Colliding Top");
        } else {
            info!("Colliding Bottom");
        };

        CollidePosition::None
    }
}
