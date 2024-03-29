use bevy::{prelude::*, utils::HashMap};

use super::{Animation, AnimationIndices, AnimationKey, AnimationMap};
use crate::components::EntityState;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut atlas_server: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<AnimationMap>,
) {
    let texture: Handle<Image> = asset_server.load("knight/Knight.png");
    let layout = atlas_server.add(TextureAtlasLayout::from_grid(
        Vec2::new(31.0, 38.0),
        18,
        1,
        None,
        None,
    ));

    let idle_animation = AnimationIndices::new(1, 7);
    let mut range: HashMap<EntityState, AnimationIndices> = HashMap::new();
    range.insert(EntityState::Idle, idle_animation);
    range.insert(EntityState::Running, AnimationIndices::new(8, 17));

    let range = Animation {
        texture,
        atlas: layout,
        indices: range,
        default: idle_animation,
    };

    animations.0.insert(AnimationKey::Player, range);
}
