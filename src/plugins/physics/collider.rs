use bevy::{
    log::info,
    math::{
        bounding::{Aabb2d, BoundingVolume},
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
        let height = size.y / 2.0;
        let width = size.x / 2.0;
        let entity = Aabb2d::new(translation.truncate(), *size / Vec2::new(2.0, 2.0));

        let closest = self.platform.closest_point(entity.center());
        let offset = entity.center() - closest;
        if offset.x.abs() > offset.y.abs() {
            if offset.x < 0.0 {
                info!("Colliding top");
            } else {
                info!("Colliding bottom");
            }
        } else if offset.y > 0.0 {
            info!("Colliding left");
        } else {
            info!("Colliding right");
        };

        CollidePosition::None
    }
}
