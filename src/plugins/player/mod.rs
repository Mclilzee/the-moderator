pub mod constants;
pub mod player_movement;

use bevy::prelude::*;

use crate::{
    bundles::character::Character,
    components::{Jumps, Player},
};

use self::constants::*;
use self::player_movement::movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, movement);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_server: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut char = (
        Character::new(PLAYER_STARING_HP, Vec2::splat(30.0)),
        Player,
        Jumps(ALLOWED_JUMPS),
    );

    let texture: Handle<Image> = asset_server.load("knight/all.png");
    let layout = atlas_server.add(TextureAtlasLayout::from_grid(
        Vec2::new(31.0, 38.0),
        5,
        4,
        None,
        None,
    ));

    char.0.movable_object.sprite_sheet.texture = texture;
    char.0.movable_object.sprite_sheet.atlas = TextureAtlas { layout, index: 1 };

    commands.spawn((char, Name::new("Player")));
}
