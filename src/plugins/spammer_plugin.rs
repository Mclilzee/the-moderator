use crate::components::{Hp, Player, Spammer, Speed};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::{self, Rng};

pub struct SpammerPlugins;

#[derive(Resource)]
struct SpammerSpawnTimer {
    timer: Timer,
}

impl Plugin for SpammerPlugins {
    fn build(&self, app: &mut App) {
        let timer = SpammerSpawnTimer {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        };
        app.insert_resource(timer)
            .add_systems(Update, spawn_spammer)
            .add_systems(Update, track_player);
    }
}

fn spawn_spammer(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpammerSpawnTimer>,
    spammers_query: Query<&Spammer>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Limit spawn to 10 spammers
    if spammers_query.iter().count() > 10 {
        return;
    }

    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.just_finished() {
        let mut random = rand::thread_rng();
        let offset = random.gen_range(-50.0..50.0);
        let screen_offset = window_query.single().width() / 2.0;
        let x = offset + f32::copysign(screen_offset + 5.0, offset);

        let sprite = SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, 0.0, 0.0),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        };

        commands.spawn((sprite, Speed(50.0), Spammer, Hp(5)));
    }
}

type SpammerQuery = (With<Spammer>, Without<Player>);
type PlayerQuery = (With<Player>, Without<Spammer>);

fn track_player(
    mut spammer_query: Query<(&mut Transform, &Speed), SpammerQuery>,
    player_query: Query<&Transform, PlayerQuery>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();

    for (mut transform, speed) in spammer_query.iter_mut() {
        let direction = player_transform.translation - transform.translation;
        let direction = direction.truncate().normalize_or_zero() * speed.0;
        transform.translation += direction.extend(0.0) * time.delta_seconds();
    }
}
