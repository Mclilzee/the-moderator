use bevy::prelude::*;

#[derive(Component, Default)]
pub enum EntityState {
    #[default]
    Grounded,
    Flying,
    Solid,
}
