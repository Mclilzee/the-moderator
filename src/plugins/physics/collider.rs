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
        let closest = self.platform.closest_point(entity.center());
        let offset = entity.center() - closest;

        println!("Closest: {closest}, Offset: {offset}");
        println!("Abs x {}, Abs y {}", offset.x.abs(), offset.y.abs());
        println!("Half Size {}", entity.half_size());

        if offset.x < 0.0 {
            return CollidePosition::Left(Vec3::new(
                self.platform.min.x - entity.half_size().x,
                translation.y,
                translation.z,
            ));
        }

        if offset.x > 0.0 {
            return CollidePosition::Right(Vec3::new(
                self.platform.max.x + entity.half_size().x,
                translation.y,
                translation.z,
            ));
        }

        if offset.y < 0.0 {
            return CollidePosition::Bottom(Vec3::new(
                translation.x,
                self.platform.min.y - entity.half_size().y,
                translation.z,
            ));
        }

        CollidePosition::Top(Vec3::new(
            translation.x,
            self.platform.max.y + entity.half_size().y,
            translation.z,
        ))
    }
}
