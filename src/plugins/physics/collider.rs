use bevy::math::{Vec2, Vec3};

pub enum CollidePosition {
    Top(Vec3),
    Bottom(Vec3),
    Left(Vec3),
    Right(Vec3),
    None,
}

pub struct PlatformCollider {
    top_left: Vec2,
    top_right: Vec2,
    bottom_left: Vec2,
    bottom_right: Vec2,
}

impl PlatformCollider {
    pub fn new(translation: &Vec3, size: &Vec2) -> Self {
        let width = size.x / 2.0;
        let height = size.y / 2.0;

        PlatformCollider {
            top_left: Vec2::new(translation.x - width, translation.y + height),
            top_right: Vec2::new(translation.x + width, translation.y + height),
            bottom_left: Vec2::new(translation.x - width, translation.y - height),
            bottom_right: Vec2::new(translation.x + width, translation.y - height),
        }
    }

    pub fn position(&self, translation: &Vec3, size: &Vec2) -> CollidePosition {
        let height = size.y / 2.0;
        let width = size.x / 2.0;
        let top_left = Vec2::new(translation.x - width, translation.y + height);
        let top_right = Vec2::new(translation.x + width, translation.y + height);
        let bottom_left = Vec2::new(translation.x - width, translation.y - height);
        let bottom_right = Vec2::new(translation.x + width, translation.y - height);

        if self.colliding_left(&top_right, &bottom_right) {
            return CollidePosition::Left(Vec3::new(self.top_left.x, translation.y, translation.z));
        }

        if self.colliding_right(&top_left, &bottom_left) {
            return CollidePosition::Right(Vec3::new(
                self.top_right.x,
                translation.y,
                translation.z,
            ));
        }

        if self.colliding_top(&bottom_left, &bottom_right) {
            return CollidePosition::Top(Vec3::new(
                translation.x,
                self.top_left.y + height,
                translation.z,
            ));
        }

        if self.colliding_bottom(&top_left, &top_right) {
            return CollidePosition::Bottom(Vec3::new(
                translation.x,
                self.bottom_left.y - height,
                translation.z,
            ));
        }

        CollidePosition::None
    }

    fn colliding_top(&self, bottom_left: &Vec2, bottom_right: &Vec2) -> bool {
        (bottom_left.x < self.top_right.x && bottom_left.y < self.top_right.y)
            || (bottom_right.x > self.top_left.x && bottom_right.y < self.top_left.y)
    }

    fn colliding_bottom(&self, top_left: &Vec2, top_right: &Vec2) -> bool {
        (top_left.x < self.bottom_right.x && top_left.y > self.bottom_right.y)
            || (top_right.x > self.bottom_left.x && top_right.y > self.bottom_left.y)
    }

    fn colliding_left(&self, top_right: &Vec2, bottom_right: &Vec2) -> bool {
        false
    }

    fn colliding_right(&self, top_left: &Vec2, bottom_left: &Vec2) -> bool {
        false
    }
}
