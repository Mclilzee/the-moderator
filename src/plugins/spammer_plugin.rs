use crate::components::{Hp, Player, Spammer, Velocity};
use bevy::prelude::*;
use rand::{self, Rng};

pub struct SpammerPlugins;

#[derive(Resource)]
struct SpammerSpawnTimer {
    timer: Timer,
}

impl Plugin for SpammerPlugins {
    fn build(&self, app: &mut App) {
        let timer = SpammerSpawnTimer {
            timer: Timer::from_seconds(0.0002, TimerMode::Repeating),
        };
        app.insert_resource(timer)
            .add_systems(Update, spawn_spammer)
            .add_systems(Update, track_player);
    }
}

fn spawn_spammer(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpammerSpawnTimer>,
    mut query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.just_finished() {
        let mut random = rand::thread_rng();
        let player_transform = query.single_mut();
        let offset = random.gen_range(-50.0..50.0);
        let spammer_pos = Vec2::new(
            player_transform.translation.x + offset + f32::copysign(20., offset),
            player_transform.translation.y,
        );

        let sprite = SpriteBundle {
            transform: Transform {
                translation: spammer_pos.extend(0.0),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        };

        commands.spawn((sprite, Velocity(Vec2::new(50.0, 0.0)), Spammer, Hp(5)));
    }
}

type SpammerQuery = (With<Spammer>, Without<Player>);
type PlayerQuery = (With<Player>, Without<Spammer>);

fn track_player(
    mut spammer_query: Query<(&mut Transform, &Velocity), SpammerQuery>,
    player_query: Query<&Transform, PlayerQuery>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();

    for (mut transform, velocity) in spammer_query.iter_mut() {
        let direction = player_transform.translation - transform.translation;
        let velocity = direction.truncate().normalize_or_zero() * velocity.0;
        transform.translation += velocity.extend(0.0) * time.delta_seconds();
    }
}
