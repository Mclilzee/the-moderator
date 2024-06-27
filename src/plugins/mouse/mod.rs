use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
struct Cursor;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn);
    }
}

fn spawn(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if camera_q.is_empty() {
        info!("Camera not found");
        return;
    } else if window_q.is_empty() {
        info!("Window not found");
        return;
    }

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
    }
}

fn move_cursor(mut evr_cursor: EventReader<CursorMoved>) {
    for ev in evr_cursor.read() {
        info!(
            "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
            ev.position.x, ev.position.y, ev.window
        )
    }
}
