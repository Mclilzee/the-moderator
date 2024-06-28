use bevy::{prelude::*, utils::HashMap};

use super::{Animation, AnimationIndices, AnimationKey, AnimationMap};
use crate::components::EntityState;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut atlas_server: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<AnimationMap>,
) {
    let texture: Handle<Image> = asset_server.load("Hammer.png");
    let layout = atlas_server.add(TextureAtlasLayout::from_grid(
        Vec2::new(32.0, 32.0),
        13,
        1,
        Some(Vec2::new(4.0, 0.0)),
        None,
    ));

    let idle_animation = AnimationIndices::new(0, 12);
    let mut range: HashMap<EntityState, AnimationIndices> = HashMap::new();
    range.insert(EntityState::Idle, idle_animation);

    let range = Animation {
        texture,
        atlas: layout,
        indices: range,
        default: idle_animation,
    };

    animations.0.insert(AnimationKey::Hammer, range);
}
