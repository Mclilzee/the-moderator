use bevy::{math::Vec2, transform::components::Transform};

pub struct Collider {
    first: Bounderies,
    second: Bounderies,
}

struct Bounderies {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
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
