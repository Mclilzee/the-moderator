use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, hello_world)
        .run();
}

pub fn hello_world() {
    println!("Hello World");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Fred".to_string())));
    commands.spawn((Person, Name("Manon".to_string())));
    commands.spawn((Person, Name("Mclilzee".to_string())));
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
