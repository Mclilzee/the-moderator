mod player_assets;
mod spammer_assets;
mod weapon_assets;
use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use crate::common_components::EntityState;

const DEFAULT_ANIMATION_TIME_MILLIS: u64 = 100;

#[derive(Resource, Default)]
pub struct AnimationMap(pub HashMap<AnimationKey, Animation>);

#[derive(Eq, Hash, PartialEq, Component, Default)]
pub enum AnimationKey {
    #[default]
    Player,
    Spammer,
    Hammer,
}

pub struct Animation {
    pub texture: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
    pub indices: HashMap<EntityState, AnimationIndices>,
    pub default: AnimationIndices,
}

#[derive(Resource)]
struct AnimationTimer(Timer);

#[derive(Clone, Copy)]
pub struct AnimationIndices {
    pub first_frame: usize,
    pub last_frame: usize,
}

impl AnimationIndices {
    fn new(first_frame: usize, last_frame: usize) -> Self {
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
            .insert_resource(AnimationTimer(Timer::new(
                Duration::from_millis(DEFAULT_ANIMATION_TIME_MILLIS),
                TimerMode::Repeating,
            )))
            .add_systems(PreStartup, player_assets::setup)
            .add_systems(PreStartup, spammer_assets::setup)
            .add_systems(PreStartup, weapon_assets::setup)
            .add_systems(Update, animate);
    }
}

fn animate(
    mut animations: Query<(&mut TextureAtlas, &EntityState, &AnimationKey)>,
    animation: Res<AnimationMap>,
    mut timer: ResMut<AnimationTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    for (mut atlas, state, key) in animations.iter_mut() {
        let player_animations = &animation.0.get(key).expect("Animation to be found");

        let frames = player_animations
            .indices
            .get(state)
            .unwrap_or(&player_animations.default);

        let mut index = atlas.index + 1;

        if atlas.index >= frames.last_frame || atlas.index < frames.first_frame {
            index = frames.first_frame;
        }

        atlas.index = index;
    }
}
