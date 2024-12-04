mod attacks;
mod player_input;

use self::player_input::input;
use super::asset_loader::AnimationEvent;
use super::asset_loader::AnimationKey;
use super::asset_loader::AnimationMap;
use crate::common_components::CollisionLayer;
use crate::common_components::EntityState;
use crate::common_components::Friendly;
use crate::utils::animate;
use crate::{bundles::actors::Actor, common_components::Damage};
use avian2d::prelude::CollidingEntities;
use avian2d::prelude::CollisionLayers;
use avian2d::prelude::LinearVelocity;
use avian2d::prelude::LockedAxes;
use avian2d::prelude::Restitution;
use avian2d::prelude::SpatialQuery;
use avian2d::prelude::SpatialQueryFilter;
use bevy::color::palettes::css::GREEN;
use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use player_input::flip_on_input;

const PLAYER_SPEED: f32 = 150.0;
const PLAYER_JUMP_HEIGHT: f32 = 300.0;
const PLAYER_STARING_HP: i32 = 100;
const PLAYER_MAX_JUMPS: u8 = 2;
pub const PLAYER_HEIGHT: f32 = 14.0;
const PLAYER_WIDTH: f32 = 6.0;
const SCORE_TEXT_SIZE: f32 = 40.0;
const PLAYER_STARTING_TRANSFORM: Transform = Transform::from_xyz(1312., 150., 10.0);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct ScoreTextUi;

#[derive(Event)]
pub struct ScoreUpdateEvent {
    pub gained_points: u32,
}

#[derive(Resource)]
pub struct Score(pub u32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_event::<ScoreUpdateEvent>()
            .add_systems(PostStartup, setup)
            .add_systems(Update, input)
            .add_systems(Update, flip_on_input)
            .add_systems(Update, grounded_detection)
            .add_systems(Update, player_score_update)
            .add_systems(Update, animate_player.run_if(on_event::<AnimationEvent>()))
            .add_plugins(attacks::AttacksPlugin);
    }
}

fn setup(
    mut commands: Commands,
    asset_loader: Res<AnimationMap>,
    camera_q: Query<Entity, With<Camera>>,
) {
    let mut actor = Actor::new(PLAYER_STARING_HP, PLAYER_WIDTH, PLAYER_HEIGHT);
    let animation = asset_loader
        .0
        .get(&AnimationKey::Player)
        .expect("Player animation were not found");

    actor.sprite_bundle.texture = animation.texture.clone();
    actor.sprite_bundle.transform = PLAYER_STARTING_TRANSFORM;
    actor.atlas = TextureAtlas {
        layout: animation.atlas.clone(),
        index: 1,
    };

    let char = (
        Player,
        Damage(5),
        Friendly,
        EntityState::Idle,
        Restitution::PERFECTLY_INELASTIC,
        LockedAxes::ROTATION_LOCKED,
        CollisionLayers::new(
            CollisionLayer::Friendly,
            [CollisionLayer::Enemy, CollisionLayer::Wall],
        ),
    );

    let player_id = commands
        .spawn((char, actor, LinearVelocity::default()))
        .id();
    let id = camera_q.single();
    commands.get_entity(id).unwrap().set_parent(player_id);
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: SCORE_TEXT_SIZE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        ScoreTextUi,
    ));
}

fn player_score_update(
    mut score: ResMut<Score>,
    mut score_text: Query<&mut Text, With<ScoreTextUi>>,
    mut event: EventReader<ScoreUpdateEvent>,
) {
    for event in event.read() {
        score.0 += event.gained_points;
    }

    let mut sections = score_text.single_mut();

    let score_section = sections.sections.first_mut().expect("Score does not exist");

    match score.0 {
        0..40 => score_section.value = score.0.to_string(),
        40..100 => {
            score_section.value = format!("Club-40: {}", score.0);
            score_section.style.color = Color::Srgba(GREEN);
        }
        _ => {
            score_section.value = format!("Moderator: {}", score.0);
            score_section.style.color = Color::Srgba(RED);
        }
    };
}

fn grounded_detection(
    spatial_query: SpatialQuery,
    mut player: Query<(&Transform, &mut EntityState), With<Player>>,
) {
    let (transform, mut state) = player.single_mut();
    if spatial_query
        .cast_ray(
            transform.translation.truncate(),
            Dir2::NEG_Y,
            PLAYER_HEIGHT,
            true,
            SpatialQueryFilter::from_mask(CollisionLayer::Wall),
        )
        .is_some()
    {
        *state = EntityState::Idle;
    }
}

fn animate_player(
    mut query: Query<(&mut TextureAtlas, &EntityState), With<Player>>,
    map: Res<AnimationMap>,
) {
    let (mut atlas, state) = query.single_mut();
    animate(&mut atlas, state, &AnimationKey::Player, &map);
}
