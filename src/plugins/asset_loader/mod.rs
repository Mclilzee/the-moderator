mod player_assets;
mod spammer_assets;
mod weapon_assets;

use crate::common_components::EntityState;
use bevy::{prelude::*, utils::HashMap};

const DEFAULT_ANIMATION_TIME_SECS: f32 = 0.1;

#[derive(Resource, Default)]
pub struct AnimationMap(pub HashMap<AnimationKey, Animation>);

#[derive(Eq, Hash, PartialEq)]
pub enum AnimationKey {
    Player,
    Spammer,
    HammerThrow,
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

#[derive(Event, Default)]
pub struct AnimationEvent;

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
            .insert_resource(AnimationTimer(Timer::from_seconds(
                DEFAULT_ANIMATION_TIME_SECS,
                TimerMode::Repeating,
            )))
            .add_event::<AnimationEvent>()
            .add_systems(PreStartup, player_assets::setup)
            .add_systems(PreStartup, spammer_assets::setup)
            .add_systems(PreStartup, weapon_assets::setup)
            .add_systems(Update, timer_tick);
    }
}

fn timer_tick(
    mut timer: ResMut<AnimationTimer>,
    time: Res<Time>,
    mut event: EventWriter<AnimationEvent>
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        event.send_default();
    }
}
