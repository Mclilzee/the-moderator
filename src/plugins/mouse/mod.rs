use bevy::{prelude::*, window::PrimaryWindow};

use crate::bundles::platforms::Platforms;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw);
    }
}

fn draw(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let mouse_position = q_windows.single().cursor_position();
    if mouse_position.is_none() {
        return;
    }
    let vec = mouse_position.unwrap();

    if buttons.pressed(MouseButton::Left) {
        commands.spawn(Platforms::cuboid(
            Color::BLUE,
            Vec2::ONE,
            Transform::from_xyz(vec.x, vec.y, 0.0),
        ));
    }
}
