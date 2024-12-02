use bevy::color::palettes::css::BLUE_VIOLET;
use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
use bevy_ecs_ldtk::LdtkIntCell;
use bevy_rapier2d::geometry::Collider;

#[derive(Default, Component)]
pub struct Ground;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct GroundBundle {
    collider: Collider,
    sprite_sheet: SpriteBundle,
    ground: Ground,
}

impl GroundBundle {
    pub fn cuboid(color: Color, size: Vec2, transform: Transform) -> Self {
        Self {
            sprite_sheet: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..default()
                },
                transform,
                ..default()
            },
            collider: Collider::cuboid(size.x / 2.0, size.y / 2.0),
            ground: Ground,
        }
    }
}

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground)
            .register_ldtk_int_cell::<GroundBundle>(1);
    }
}

fn spawn_ground(mut commands: Commands) {
    commands.spawn((GroundBundle::cuboid(
        BLUE_VIOLET.into(),
        Vec2::new(8000.0, 20.0),
        Transform::from_xyz(-100.0, -10.0, 0.0),
    ),));
}
