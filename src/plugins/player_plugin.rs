use super::assets_plugin::{AnimationType, AssetsLoader};
use crate::bundles::character::Character;
use crate::{
    components::{Player, Velocity},
    consts::GRAVITY_SPEED,
};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 50.0;
const PLAYER_JUMP_HEIGHT: f32 = 200.0;
const PLAYER_STARING_HP: i32 = 100;
const ALLOWED_JUMPS: i32 = 2;

#[derive(Component)]
struct Jumps(i32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, (movement, animate_sprite).chain());
    }
}

fn spawn_player(mut commands: Commands, asset_loader: Res<AssetsLoader>) {
    let mut char = (
        Character::new(PLAYER_STARING_HP, Vec2::splat(30.0)),
        Player,
        Jumps(ALLOWED_JUMPS),
    );

    let animation_atlas = asset_loader.player_textures.get(&AnimationType::Idle);
    if let Some(sheet) = animation_atlas {
        char.0.movable_object.sprite_sheet.texture_atlas = sheet.texture_atlas.to_owned();
    }

    commands.spawn(char);
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

fn animate_sprite(
    mut player_query: Query<&mut TextureAtlasSprite, With<Player>>,
    input: Res<Input<KeyCode>>,
    asset_loader: Res<AssetsLoader>,
) {
    let player_texture = player_query.single_mut();
    if input.any_pressed([KeyCode::Left, KeyCode::D]) {}
}
