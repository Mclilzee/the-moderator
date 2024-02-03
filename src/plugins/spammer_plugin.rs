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
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        };
        app.insert_resource(timer)
            .add_systems(Update, spawn_spammer);
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
        let x = player_transform.translation.x + 20. + random.gen_range(0.0..20.0);
        let y = player_transform.translation.y;

        let sprite = SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            visibility: Visibility::Visible,
            ..default()
        };

        commands.spawn((sprite, Velocity(Vec2::new(50.0, 0.0)), Spammer, Hp(5)));
    }
}
