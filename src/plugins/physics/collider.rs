use bevy::math::Vec2;

pub struct PlatformCollider {
    entity: Bounderies,
    platform: Bounderies,
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
            entity: Bounderies::new(entity_transform, entity_size),
            platform: Bounderies::new(platform_transform, platform_size),
        }
    }

    pub fn entity_position(&self) -> Vec2 {}

    fn entity_between_left_and_right(&self) -> bool {
        self.entity.right > self.platform.left && self.entity.left < self.platform.right
    }

    fn entity_between_top_and_bottom(&self) -> bool {
        self.entity.top > self.platform.bottom && self.entity.bottom < self.platform.top
    }

    fn entity_colliding_top(&self) -> bool {
        self.entity.bottom < self.platform.top && self.entity.top > self.platform.top
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
