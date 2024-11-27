mod animation;
mod attacks;
mod player_input;

use self::{animation::animate, player_input::input};
use super::asset_loader::AnimationKey;
use super::asset_loader::AnimationMap;
use super::platform::Platform;
use crate::common_components::{EntityState, Jumps};
use crate::AnimationEvent;
use crate::{bundles::actors::Actor, common_components::Damage};
use animation::flip_on_input;
use bevy::prelude::*;
use bevy_rapier2d::dynamics::LockedAxes;
use bevy_rapier2d::geometry::{CollisionGroups, Group};
use bevy_rapier2d::plugin::RapierContext;

pub const PLAYER_SPEED: f32 = 150.0;
pub const PLAYER_JUMP_HEIGHT: f32 = 300.0;
pub const PLAYER_STARING_HP: i32 = 100;
pub const PLAYER_MAX_JUMPS: u8 = 2;
pub const PLAYER_HEIGHT: f32 = 17.0;
pub const PLAYER_WIDTH: f32 = 7.0;
const SCORE_TEXT_SIZE: f32 = 40.0;

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
            .add_systems(Update, animate.run_if(on_event::<AnimationEvent>()))
            .add_systems(Update, flip_on_input)
            .add_systems(Update, ground_collision)
            .add_systems(Update, player_score_update)
            .add_plugins(attacks::AttacksPlugin);
    }
}

fn setup(
    mut commands: Commands,
    asset_loader: Res<AnimationMap>,
    camera_q: Query<Entity, With<Camera>>,
) {
    let mut char = (
        Actor::new(PLAYER_STARING_HP, PLAYER_WIDTH, PLAYER_HEIGHT),
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
        Player,
        Damage(5),
        Jumps {
            current: 0,
            max: PLAYER_MAX_JUMPS,
        },
        EntityState::Idle,
        LockedAxes::ROTATION_LOCKED,
    );

    let animation = asset_loader
        .0
        .get(&AnimationKey::Player)
        .expect("Player animation were not found");

    char.0.sprite_bundle.texture = animation.texture.clone();
    char.0.atlas = TextureAtlas {
        layout: animation.atlas.clone(),
        index: 1,
    };

    let player_id = commands.spawn((char, Name::new("Player"))).id();
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

fn ground_collision(
    mut player: Query<(Entity, &Transform, &mut Jumps), With<Player>>,
    platforms: Query<(Entity, &Transform), With<Platform>>,
    rapier_context: Res<RapierContext>,
) {
    let (p_id, p_transform, mut jumps) = player.single_mut();
    for (platform_id, platform_transform) in platforms.iter() {
        if rapier_context.contact_pair(p_id, platform_id).is_some()
            && p_transform.translation.y > platform_transform.translation.y
        {
            jumps.current = 0;
        }
    }
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
        40..100 => score_section.value = format!("Club-40: {}", score.0),
        _ => score_section.value = format!("Moderator: {}", score.0),
    };
}
