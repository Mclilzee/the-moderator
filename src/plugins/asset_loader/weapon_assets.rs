use bevy::{prelude::*, utils::HashMap};

use super::{Animation, AnimationIndices, AnimationKey, AnimationMap};
use crate::common_components::EntityState;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut atlas_server: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<AnimationMap>,
) {
    //let texture: Handle<Image> = asset_server.load("Hammer.png");
    //let layout = atlas_server.add(TextureAtlasLayout::from_grid(
    //    UVec2::new(32, 32),
    //    13,
    //    1,
    //    None,
    //    None,
    //));
    //
    //let idle_animation = AnimationIndices::new(0, 12);
    //let mut range: HashMap<EntityState, AnimationIndices> = HashMap::new();
    //range.insert(EntityState::Idle, idle_animation);
    //
    //let range = Animation {
    //    texture,
    //    atlas: layout,
    //    indices: range,
    //    default: idle_animation,
    //};
    //
    //animations.0.insert(AnimationKey::HammerThrow, range);
}
