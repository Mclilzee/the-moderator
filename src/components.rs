use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Hp(pub i32);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Spammer;
