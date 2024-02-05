use crate::{
    components::{Character, Hp, Player, Velocity},
    consts::GRAVITY_SPEED,
};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 160.0;
const PLAYER_JUMP_HEIGHT: f32 = 500.0;
const PLAYER_STARING_HP: i32 = 100;
const PLAYER_WIDTH: f32 = 20.0;
const PLAYER_HEIGHT: f32 = 40.0;
const ALLOWED_JUMPS: i32 = 3;

#[derive(Component)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Jumps(i32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, movement);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture: Handle<Image> = asset_server.load("knight/_Idle.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        texture,
        Vec2::splat(80.0),
        1,
        1,
        Some(Vec2::splat(40.0)),
        None, // Some(Vec2::new(20.0, 20.0)),
    ));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(2.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    });

    commands.spawn((
        Character {
            sprite_sheet: SpriteSheetBundle {
                texture_atlas,
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(24.0)),
                    ..default()
                },
                ..default()
            },
            hp: Hp(PLAYER_STARING_HP),
            velocity: Velocity::default(),
        },
        Player,
        Jumps(ALLOWED_JUMPS),
    ));
}

fn movement(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Jumps, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (mut player_velocity, mut available_jumps, mut transform) = query.single_mut();
    let mut velocity = Vec3::new(0.0, player_velocity.translation.y, 0.0);

    if keys.pressed(KeyCode::Right) {
        velocity.x = PLAYER_SPEED;
    }

    if keys.pressed(KeyCode::Left) {
        velocity.x = -PLAYER_SPEED;
    }

    if keys.just_pressed(KeyCode::Up) && available_jumps.0 >= 1 {
        velocity.y = PLAYER_JUMP_HEIGHT + GRAVITY_SPEED;
        available_jumps.0 -= 1;
    }

    velocity.y -= GRAVITY_SPEED;
    player_velocity.translation = velocity;

    transform.translation += player_velocity.translation * time.delta_seconds();

    if transform.translation.y < 0.0 {
        player_velocity.translation.y = 0.0;
        transform.translation.y = 0.0;
        available_jumps.0 = ALLOWED_JUMPS;
    }
}
