use bevy::math::Vec2;

pub enum CollidePosition {
    Top,
    Bottom,
    Left,
    Right,
}

pub struct PlatformCollider {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl PlatformCollider {
    pub fn new(transform: &Vec2, size: &Vec2) -> Self {
        let height_offset = size.x / 2.0;
        let width_offset = size.y / 2.0;

        PlatformCollider {
            left: transform.x - width_offset,
            right: transform.x + width_offset,
            top: transform.y + height_offset,
            bottom: transform.y - height_offset,
        }
    }

    pub fn position(&self, transform: &Vec2, size: &Vec2) -> Option<CollidePosition> {
        let height = size.y / 2.0;
        let width = size.x / 2.0;
        let left = transform.x - width;
        let right = transform.x + width;
        let top = transform.y + height;
        let bottom = transform.y - height;

        if self.between_left_and_right(left, right) {
            if self.between_top_and_bottom(top, bottom) || self.colliding_top(top, bottom) {}
        }

        None
    }

    fn between_left_and_right(&self, left: f32, right: f32) -> bool {
        right > self.left && left < self.right
    }

    fn between_top_and_bottom(&self, top: f32, bottom: f32) -> bool {
        top > self.bottom && bottom < self.top
    }

    fn colliding_top(&self, top: f32, bottom: f32) -> bool {
        bottom < self.top && top > self.top
    }

    fn colliding_bottom(&self, top: f32, bottom: f32) -> bool {
        top > self.bottom && bottom < self.bottom
    }

    fn colliding_left(&self, left: f32, right: f32) -> bool {
        right > self.left && left < self.left
    }

    fn colliding_right(&self, left: f32, right: f32) -> bool {
        left < self.right && right > self.right
    }
}
