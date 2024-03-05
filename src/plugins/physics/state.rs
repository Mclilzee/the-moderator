use bevy::prelude::*;

#[derive(Component)]
pub enum State {
    Grounded,
    Flying,
    Solid,
}
