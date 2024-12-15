mod attacks;
mod player_input;

use self::player_input::input;
use super::asset_loader::AnimationEvent;
use super::asset_loader::AnimationKey;
use super::asset_loader::AnimationMap;
use crate::bundles::actors::Actor;
use crate::common_components::CollisionLayer;
use crate::common_components::EntityState;
use crate::common_components::Friendly;
use crate::utils::animate;
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
const PLAYER_JUMP_HEIGHT: f32 = 500.0;
const PLAYER_STARING_HP: i32 = 100;
pub const PLAYER_LENGTH: f32 = 14.0;
const PLAYER_RADIUS: f32 = 6.0;
const SCORE_TEXT_SIZE: f32 = 40.0;
const PLAYER_STARTING_POSITION: Vec3 = Vec3::new(1312., 160., 10.0);

#[derive(Event, Default)]
pub struct JumpEvent;

#[derive(Event, Default)]
pub struct DoubleJumpEvent;

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
            .add_event::<JumpEvent>()
            .add_event::<DoubleJumpEvent>()
            .add_systems(PostStartup, setup)
            .add_systems(Update, input)
            .add_systems(Update, flip_on_input)
            .add_systems(Update, grounded_detection)
            .add_systems(Update, player_score_update)
            .add_systems(Update, animate_player.run_if(on_event::<AnimationEvent>))
            .add_plugins(attacks::AttacksPlugin);
    }
}

fn setup(
    mut commands: Commands,
    asset_loader: Res<AnimationMap>,
    mut camera_q: Query<(Entity, &mut Transform), With<Camera>>,
    asset_server: Res<AssetServer>
) {
    let animation = asset_loader
        .0
        .get(&AnimationKey::Player)
        .expect("Player animation were not found");
    let (camera_id, mut camera_transform) = camera_q.single_mut();
    camera_transform.translation.y = 100.0;
    commands
        .spawn((
            Player,
            Sprite::from_atlas_image(
                animation.texture.clone(),
                TextureAtlas {
                    layout: animation.atlas.clone(),
                    index: 1,
                },
            ),
            Transform::from_translation(PLAYER_STARTING_POSITION),
            LinearVelocity::default(),
            Friendly,
            Actor::new(PLAYER_STARING_HP, PLAYER_RADIUS, PLAYER_LENGTH),
            EntityState::Idle,
            Restitution::PERFECTLY_INELASTIC,
            LockedAxes::ROTATION_LOCKED,
            CollisionLayers::new(
                CollisionLayer::Friendly,
                [CollisionLayer::Enemy, CollisionLayer::Wall],
            ),
        ))
        .add_child(camera_id);

    commands.spawn((
        Text::new("0"),
        TextFont::from_font_size(SCORE_TEXT_SIZE).with_font(asset_server.load("score-font.ttf")),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ScoreTextUi,
    ));
}

fn player_score_update(
    mut score: ResMut<Score>,
    mut score_text: Query<(&mut Text, &mut TextColor), With<ScoreTextUi>>,
    mut event: EventReader<ScoreUpdateEvent>,
) {
    for event in event.read() {
        score.0 += event.gained_points;
    }
    let (mut text, mut color) = score_text.single_mut();

    match score.0 {
        0..40 => text.0 = score.0.to_string(),
        40..100 => {
            text.0 = format!("Club-40: {}", score.0);
            color.0 = Color::Srgba(GREEN);
        }
        _ => {
            text.0 = format!("Moderator: {}", score.0);
            color.0 = Color::Srgba(RED);
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
            PLAYER_LENGTH,
            true,
            &SpatialQueryFilter::from_mask(CollisionLayer::Wall),
        )
        .is_some()
    {
        *state = EntityState::Idle;
    }
}

fn animate_player(
    mut query: Query<(&mut Sprite, &EntityState), With<Player>>,
    map: Res<AnimationMap>,
) {
    let (mut sprite, state) = query.single_mut();
    if let Some(atlas) = sprite.texture_atlas.as_mut() {
        animate(atlas, state, &AnimationKey::Player, &map);
    }
}
