use crate::components::{Hp, Player, Spammer, Velocity};
use bevy::prelude::*;

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
    mut query: Query<&Transform, With<Player>>,
    spawn_timer: Res<SpammerSpawnTimer>,
) {
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let player_transform = query.single_mut();
    let x = player_transform.translation.x + 20.;
    let y = player_transform.translation.y + 20.;

    let sprite = SpriteBundle {
        transform: Transform::from_xyz(x, y, 0.0),
        visibility: Visibility::Visible,
        ..default()
    };

    commands.spawn((
        sprite,
        Velocity {
            value: Vec2::new(50.0, 0.0),
        },
        Spammer,
        Hp(5),
    ));
}
