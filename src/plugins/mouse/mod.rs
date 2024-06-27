use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
struct Cursor;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn)
            .add_systems(Update, move_cursor);
    }
}

fn spawn(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window_q.single();
    if let Some(vec) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin)
    {
        commands.spawn((
            Cursor,
            SpriteSheetBundle {
                transform: Transform {
                    translation: vec,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                ..default()
            },
        ));

        info!("spawn successfully");
    }
}

fn move_cursor(
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut cursor_q: Query<&mut Transform, With<Cursor>>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window_q.single();
    if let Some(vec) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin)
    {
        if let Ok(mut transform) = cursor_q.get_single_mut() {
            transform.translation = vec;
            info!("{vec:?}");
        }
    }
}
