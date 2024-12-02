use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
use bevy_ecs_ldtk::LdtkIntCell;
use bevy_rapier2d::geometry::Collider;

#[derive(Default, Component)]
pub struct Ground;

#[derive(Bundle, LdtkIntCell)]
pub struct GroundBundle {
    collider: Collider,
    sprite_sheet: SpriteBundle,
    ground: Ground,
}

impl Default for GroundBundle {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(2.0, 2.0),
            ..default()
        }
    }
}

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<GroundBundle>(1);
    }
}
