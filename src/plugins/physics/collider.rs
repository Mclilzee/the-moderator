use bevy::math::Vec2;

pub struct PlatformCollider {
    entity: Option<Bounderies>,
    platform: Bounderies,
}

struct Bounderies {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    height_offset: f32,
    width_offset: f32,
}

impl PlatformCollider {
    pub fn new(transform: &Vec2, size: &Vec2) -> Self {
        PlatformCollider {
            entity: None,
            platform: Bounderies::new(transform, size),
        }
    }

    pub fn entity(&mut self, transform: &Vec2, size: &Vec2) {
        self.entity = Some(Bounderies::new(transform, size));
    }

    pub fn entity_position(&self) -> Vec2 {}

    fn between_left_and_right(&self) -> bool {
        match &self.entity {
            Some(entity) => entity.right > self.platform.left && entity.left < self.platform.right,
            None => false,
        }
    }

    fn between_top_and_bottom(&self) -> bool {
        match &self.entity {
            Some(entity) => entity.top > self.platform.bottom && entity.bottom < self.platform.top,
            None => false,
        }
    }

    fn colliding_top(&self) -> bool {
        match &self.entity {
            Some(entity) => entity.bottom < self.platform.top && entity.top > self.platform.top,
            None => false,
        }
    }

    fn colliding_bottom(&self) -> bool {
        match &self.entity {
            Some(entity) => {
                entity.top > self.platform.bottom && entity.bottom < self.platform.bottom
            }
            None => false,
        }
    }

    fn colliding_left(&self) -> bool {
        match &self.entity {
            Some(entity) => entity.right > self.platform.left && entity.left < self.platform.left,
            None => false,
        }
    }

    fn colliding_right(&self) -> bool {
        match &self.entity {
            Some(entity) => entity.left < self.platform.right && entity.right > self.platform.right,
            None => false,
        }
    }
}

impl Bounderies {
    fn new(transform: &Vec2, size: &Vec2) -> Self {
        let height_offset = size.x / 2.0;
        let width_offset = size.y / 2.0;

        Bounderies {
            left: transform.x - width_offset,
            right: transform.x + width_offset,
            top: transform.y + height_offset,
            bottom: transform.y - height_offset,
            height_offset,
            width_offset,
        }
    }
}
