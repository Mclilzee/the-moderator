use bevy::{prelude::*, utils::HashMap};

use super::{Animation, AnimationIndices, AnimationKey, AnimationMap};
use crate::common_components::EntityState;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut atlas_server: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<AnimationMap>,
    textures: ResMut<Assets<Image>>,
) {
    let mut builder = TextureAtlasBuilder::default();
    let texture: Handle<Image> = asset_server.load("fred/idle.png");
    builder.add_texture(Some(texture.id()), texture);
    let texture: Handle<Image> = asset_server.load("fred/run.png");
    builder.add_texture(Some(texture.id()), textures.get(texture.id()).unwrap());

    let (layout, image) = builder.build().unwrap();

    let idle_animation = AnimationIndices::new(0, 10);
    let run_animation = AnimationIndices::new(10, 22);
    let mut range: HashMap<EntityState, AnimationIndices> = HashMap::new();
    range.insert(EntityState::Idle, idle_animation);
    range.insert(EntityState::Running, run_animation);

    let range = Animation {
        texture: asset_server.add(image),
        atlas: atlas_server.add(layout),
        indices: range,
        default: idle_animation,
    };

    animations.0.insert(AnimationKey::Player, range);
}
