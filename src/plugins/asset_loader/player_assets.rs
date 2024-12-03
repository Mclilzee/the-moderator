use bevy::{prelude::*, utils::HashMap};

use super::{Animation, AnimationIndices, AnimationKey, AnimationMap};
use crate::common_components::EntityState;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut atlas_server: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<AnimationMap>,
) {
    let texture: Handle<Image> = asset_server.load("fred.png");
    let atlas = atlas_server.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        12,
        3,
        None,
        None,
    ));
    let idle_animation = AnimationIndices::new(0, 10);
    let run_animation = AnimationIndices::new(12, 23);
    let jump_animation = AnimationIndices::new(22, 23);
    let fall_animation = AnimationIndices::new(23, 24);
    let double_jump = AnimationIndices::new(24, 30);

    let mut range: HashMap<EntityState, AnimationIndices> = HashMap::new();
    range.insert(EntityState::Idle, idle_animation);
    range.insert(EntityState::Running, run_animation);
    range.insert(EntityState::Jumping, jump_animation);
    range.insert(EntityState::DoubleJumping, double_jump);
    range.insert(EntityState::Falling, fall_animation);

    let range = Animation {
        texture,
        atlas,
        indices: range,
        default: idle_animation,
    };

    animations.0.insert(AnimationKey::Player, range);
}
