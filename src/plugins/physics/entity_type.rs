use bevy::prelude::*;

#[derive(Component)]
pub enum EntityType {
    Grounded,
    Flying,
    Solid,
}
