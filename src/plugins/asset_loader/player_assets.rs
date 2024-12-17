use bevy::{prelude::*, utils::HashMap};

use super::{Animation, AnimationIndices, AnimationKey, AnimationMap};
use crate::common_components::EntityState;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut atlas_server: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<AnimationMap>,
) {
    let texture: Handle<Image> = asset_server.load("./fred.png");
    let atlas = atlas_server.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        12,
        3,
        None,
        None,
    ));
    let idle_animation = AnimationIndices::new(0, 11);
    let run_animation = AnimationIndices::new(12, 24);
    let fall_animation = AnimationIndices::new(24, 25);
    let jump_animation = AnimationIndices::new(25, 26);
    let double_jump = AnimationIndices::new(26, 32);

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
