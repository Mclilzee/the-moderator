use crate::{
    components::{Character, Hp, Player, Velocity},
    consts::GRAVITY_SPEED,
};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 50.0;
const PLAYER_JUMP_HEIGHT: f32 = 200.0;
const PLAYER_STARING_HP: i32 = 100;
const ALLOWED_JUMPS: i32 = 2;

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
    let texture: Handle<Image> = asset_server.load("knight/idle.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        texture,
        Vec2::new(21.0, 38.0),
        1,
        1,
        Some(Vec2::new(99.0, 0.0)),
        None,
    ));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(21.0, 38.0)),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Character {
            sprite_sheet: SpriteSheetBundle {
                texture_atlas,
                sprite: TextureAtlasSprite::default(),
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

    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        velocity.x = PLAYER_SPEED;
    }

    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        velocity.x = -PLAYER_SPEED;
    }

    if keys.any_just_pressed([KeyCode::Up, KeyCode::W, KeyCode::Space]) && available_jumps.0 >= 1 {
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
