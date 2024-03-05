use bevy::prelude::*;

#[derive(Component)]
pub enum EntityState {
    Grounded,
    Flying,
    Solid,
}
