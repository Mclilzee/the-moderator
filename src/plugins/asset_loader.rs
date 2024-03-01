use std::ops::Range;

use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct AnimationMap(HashMap<ComponentType, Animation>);

pub enum ComponentType {
    Player,
    Spammer,
}

pub enum AnimationType {
    Idle,
    Running,
}

pub struct Animation {
    atlas: TextureAtlasLayout,
    range: HashMap<AnimationType, Range<u32>>,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AnimationMap::default());
    }
}

fn setup(asset_server: Res<AssetServer>, mut atlas_server: ResMut<Assets<TextureAtlasLayout>>) {
    let texture: Handle<Image> = asset_server.load("knight/Knight-Sheet.png");
    let layout = atlas_server.add(TextureAtlasLayout::from_grid(
        Vec2::new(31.0, 38.0),
        18,
        1,
        None,
        None,
    ));
}
