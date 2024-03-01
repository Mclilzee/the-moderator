use bevy::prelude::*;

#[derive(Resource)]
pub struct AssetMap {
}


pub enum AnimationType {
    Player,
    Spammer
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
    }
}
