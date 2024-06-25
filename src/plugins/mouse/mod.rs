use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
struct Cursor;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_cursor);
    }
}

fn show_cursor(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    cursor_entity_q: Query<Entity, With<Cursor>>,
) {
    if window_q.is_empty() || camera_q.is_empty() {
        return;
    }

    for id in cursor_entity_q.iter() {
        commands.entity(id).despawn();
    }

    let (camera, camera_transform) = camera_q.single();
    let window = window_q.single();
    if let Some(vec) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        commands.spawn((
            Cursor,
            SpriteSheetBundle {
                transform: Transform::from_xyz(vec.x, vec.y, 0.0),
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                ..default()
            },
        ));
    }
}
