use crate::components::{Hp, Player, Velocity};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle::default(),
        Hp(100),
        Player,
        Velocity(Vec2::new(60.0, 0.0)),
    ));
}

fn move_player(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
    time: Res<Time>,
) {
    let (mut transform, velocity) = query.single_mut();

    if keys.pressed(KeyCode::Left) {
        transform.translation.x -= velocity.0.x * time.delta_seconds();
    } else if keys.pressed(KeyCode::Right) {
        transform.translation.x += velocity.0.x * time.delta_seconds();
    }
}
