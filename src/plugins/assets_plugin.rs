use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Eq, PartialEq, Hash)]
pub enum AnimationType {
    Idle,
    MovingLeft,
    MovingRight,
}

#[derive(Resource)]
pub struct AssetsLoader {
    pub player_textures: HashMap<AnimationType, AnimationCollection>,
}

#[derive(Component)]
pub struct AnimationCollection {
    pub timer: Timer,
    pub texture_atlas: Handle<TextureAtlasLayout>,
}

impl AnimationCollection {
    fn new(texture_atlas: Handle<TextureAtlasLayout>, interval: f32) -> Self {
        Self {
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            texture_atlas,
        }
    }
}

pub struct AssetsLoaderPlugin;
impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetsLoader {
            player_textures: HashMap::new(),
        })
        .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut loader: ResMut<AssetsLoader>,
) {
    loader.player_textures = HashMap::new();

    let texture: Handle<Image> = asset_server.load("knight/full_slide.png");
    let mut layout = TextureAtlasLayout::from_grid(
        Vec2::new(38.0, 24.0),
        3,
        1,
        Some(Vec2::new(82.0, 0.0)),
        Some(Vec2::new(45.0, 56.0)),
    );

    let texture_atlas = texture_atlases.add(layout);

    println!("{:?}", texture_atlas);

    loader.player_textures.insert(
        AnimationType::Idle,
        AnimationCollection::new(texture_atlas, 0.2),
    );
}
