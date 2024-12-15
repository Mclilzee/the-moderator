mod player_assets;
mod spammer_assets;
mod hammer_throw_assets;
mod fire_slash_assets;
mod death_effect;

use crate::common_components::EntityState;
use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

const DEFAULT_ANIMATION_TIME_SECS: f32 = 0.1;

#[derive(Resource, Default)]
pub struct AnimationMap(pub HashMap<AnimationKey, Animation>);

#[derive(Eq, Hash, PartialEq)]
pub enum AnimationKey {
    Player,
    Spammer,
    HammerThrow,
    FireSlash,
    DeathEffect
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
            .add_systems(PreStartup, hammer_throw_assets::setup)
            .add_systems(PreStartup, fire_slash_assets::setup)
            .add_systems(PreStartup, death_effect::setup)
            .add_systems(PreStartup, load_ldtk)
            .insert_resource(LevelSelection::index(0))
            .add_systems(Update, timer_tick);
    }
}

fn load_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: bevy_ecs_ldtk::LdtkProjectHandle { handle: asset_server.load("world.ldtk") },
        ..default()
    });
}

fn timer_tick(
    mut timer: ResMut<AnimationTimer>,
    time: Res<Time>,
    mut event: EventWriter<AnimationEvent>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        event.send_default();
    }
}
