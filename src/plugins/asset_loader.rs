use std::ops::Range;

use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct AnimationMap(HashMap<AnimationKey, Animation>);

#[derive(Eq, Hash, PartialEq)]
pub enum AnimationKey {
    Player,
    Spammer,
}

#[derive(Eq, Hash, PartialEq)]
pub enum AnimationType {
    Idle,
    Running,
}

pub struct Animation {
    pub texture: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
    pub range: HashMap<AnimationType, Range<u32>>,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AnimationMap::default())
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(
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
    let mut range: HashMap<AnimationType, Range<u32>> = HashMap::new();
    range.insert(AnimationType::Idle, 1..7);

    let range = Animation {
        texture,
        atlas: layout,
        range,
    };

    animations.0.insert(AnimationKey::Player, range);
}
