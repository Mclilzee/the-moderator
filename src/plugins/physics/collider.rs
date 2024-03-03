use bevy::math::{Vec2, Vec3};

pub enum CollidePosition {
    Top(Vec3),
    Bottom(Vec3),
    Left(Vec3),
    Right(Vec3),
    None,
}

struct Aabb {
    top_left: Vec2,
    top_right: Vec2,
    bottom_left: Vec2,
    bottom_right: Vec2,
    center: Vec2,
}

impl Aabb {
    pub fn new(translation: &Vec3, size: &Vec2) -> Self {
        let width = size.x / 2.0;
        let height = size.y / 2.0;

        Aabb {
            top_left: Vec2::new(translation.x - width, translation.y + height),
            top_right: Vec2::new(translation.x + width, translation.y + height),
            bottom_left: Vec2::new(translation.x - width, translation.y - height),
            bottom_right: Vec2::new(translation.x + width, translation.y - height),
            center: translation.truncate(),
        }
    }
}

pub struct PlatformCollider {
    entity: Option<Aabb>,
    platform: Aabb,
}

impl PlatformCollider {
    pub fn new(translation: &Vec3, size: &Vec2) -> Self {
        PlatformCollider {
            entity: None,
            platform: Aabb::new(translation, size),
        }
    }

    pub fn position(&self, translation: &Vec3, size: &Vec2) -> CollidePosition {
        let height = size.y / 2.0;
        let width = size.x / 2.0;

        if self.colliding_left() {
            return CollidePosition::Left(Vec3::new(
                self.platform.top_left.x,
                translation.y,
                translation.z,
            ));
        }

        if self.colliding_right() {
            return CollidePosition::Right(Vec3::new(
                self.platform.top_right.x,
                translation.y,
                translation.z,
            ));
        }

        if self.colliding_top() {
            return CollidePosition::Top(Vec3::new(
                translation.x,
                self.platform.top_left.y + height,
                translation.z,
            ));
        }

        if self.colliding_bottom() {
            return CollidePosition::Bottom(Vec3::new(
                translation.x,
                self.platform.bottom_left.y - height,
                translation.z,
            ));
        }

        CollidePosition::None
    }

    fn colliding_top(&self) -> bool {
        match &self.entity {
            Some(entity) => {
                (entity.center.y > self.platform.center.y
                    && entity.bottom_left.x < self.platform.top_right.x
                    && entity.bottom_left.y < self.platform.top_right.y)
                    || (entity.bottom_right.x > self.platform.top_left.x
                        && entity.bottom_right.y < self.platform.top_left.y)
            }
            None => false,
        }
    }

    fn colliding_bottom(&self) -> bool {
        (top_left.x < self.bottom_right.x && top_left.y > self.bottom_right.y)
            || (top_right.x > self.bottom_left.x && top_right.y > self.bottom_left.y)
    }

    fn colliding_left(&self) -> bool {
        false
    }

    fn colliding_right(&self) -> bool {
        false
    }
}
