use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, (update_people, greet_people).chain());
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Fred".to_string())));
    commands.spawn((Person, Name("Manon".to_string())));
    commands.spawn((Person, Name("Mclilzee".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    query.iter().for_each(|name| println!("hello {}!", name.0))
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    if let Some(mut name) = query.iter_mut().find(|name| name.0 == "Manon") {
        name.0 = "Other".to_string();
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
