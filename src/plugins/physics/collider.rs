use bevy::math::Vec2;

pub struct PlatformCollider {
    first: Bounderies,
    second: Bounderies,
}

struct Bounderies {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl PlatformCollider {
    pub fn new(
        entity_transform: &Vec2,
        entity_size: &Vec2,
        platform_transform: &Vec2,
        platform_size: &Vec2,
    ) -> Self {
        PlatformCollider {
            first: Bounderies::new(entity_transform, entity_size),
            second: Bounderies::new(platform_transform, platform_size),
        }
    }
}

impl Bounderies {
    fn new(transform: &Vec2, size: &Vec2) -> Self {
        let height = size.x / 2.0;
        let width = size.y / 2.0;

        Bounderies {
            left: transform.x - width,
            right: transform.x + width,
            top: transform.y + height,
            bottom: transform.y - height,
        }
    }
}
