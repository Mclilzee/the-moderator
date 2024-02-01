use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (update_people, greet_people, move_spammer).chain());
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Fred".to_string())));
    commands.spawn((Person, Name("Spammer".to_string()), Position::new(0.0, 0.0)));
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<(&Name, Option<&Position>), With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for person in query.iter() {
            println!("Name: {}", person.0 .0);
            if let Some(position) = person.1 {
                println!("X: {}, Y: {}", position.x, position.y);
            }
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    if let Some(mut name) = query.iter_mut().find(|name| name.0 == "Manon") {
        name.0 = "Other".to_string();
    }
}

fn move_spammer(time: Res<Time>, mut query: Query<&mut Position>) {
    if let Some(mut position) = query.iter_mut().last() {
        position.x += time.delta().as_secs_f32();
        position.y += time.delta().as_secs_f32();
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

impl Position {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);
