use bevy::math::{Vec2, Vec3};

pub enum CollidePosition {
    Top(Vec3),
    Bottom(Vec3),
    Left(Vec3),
    Right(Vec3),
    None,
}

pub struct PlatformCollider {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl PlatformCollider {
    pub fn new(transform: &Vec3, size: &Vec2) -> Self {
        let height_offset = size.x / 2.0;
        let width_offset = size.y / 2.0;

        PlatformCollider {
            left: transform.x - width_offset,
            right: transform.x + width_offset,
            top: transform.y + height_offset,
            bottom: transform.y - height_offset,
        }
    }

    pub fn position(&self, transform: &Vec3, size: &Vec2) -> CollidePosition {
        let height = size.y / 2.0;
        let width = size.x / 2.0;
        let left = transform.x - width;
        let right = transform.x + width;
        let top = transform.y + height;
        let bottom = transform.y - height;

        if self.between_left_and_right(left, right) {
            if self.between_top_and_bottom(top, bottom) || self.colliding_top(top, bottom) {
                CollidePosition::Top(Vec3::new(0.0, self.top + height, 0.0));
            }

            if self.colliding_bottom(top, bottom) {
                CollidePosition::Bottom(Vec3::new(0.0, self.bottom - height, 0.0));
            }
        } else if self.between_top_and_bottom(top, bottom) {
            if self.colliding_left(left, right) {
                CollidePosition::Left(Vec3::new(self.left - width, 0.0, 0.0));
            }

            if self.colliding_right(left, right) {
                CollidePosition::Right(Vec3::new(self.right + width, 0.0, 0.0));
            }
        }

        CollidePosition::None
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
