use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct AnimationMap(pub HashMap<AnimationKey, Animation>);

#[derive(Eq, Hash, PartialEq)]
pub enum AnimationKey {
    Player,
}

#[derive(Eq, Hash, PartialEq)]
pub enum AnimationType {
    Idle,
    Running,
}

pub struct Animation {
    pub texture: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
    pub range: HashMap<AnimationType, AnimationIndices>,
}

pub struct AnimationIndices {
    pub first_frame: u32,
    pub last_frame: u32,
}

impl AnimationIndices {
    fn new(first_frame: u32, last_frame: u32) -> Self {
        AnimationIndices {
            first_frame,
            last_frame,
        }
    }
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
    let mut range: HashMap<AnimationType, AnimationIndices> = HashMap::new();
    range.insert(AnimationType::Idle, AnimationIndices::new(1, 7));
    range.insert(AnimationType::Running, AnimationIndices::new(8, 16));

    let range = Animation {
        texture,
        atlas: layout,
        range,
    };

    animations.0.insert(AnimationKey::Player, range);
}
