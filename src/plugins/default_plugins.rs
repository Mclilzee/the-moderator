use bevy::{
    asset::AssetLoader,
    prelude::*,
    window::{Cursor, PrimaryWindow},
};

use super::asset_loader;

pub struct CustomDefaultPlugin;

#[derive(Component)]
struct CustomCursor;

#[derive(Resource)]
struct CursorDirection(Vec2);

impl Plugin for CustomDefaultPlugin {
    fn build(&self, app: &mut App) {
        let window = create_window();
        let default_plugins = DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(window),
                ..default()
            })
            .build();

        app.add_plugins(default_plugins)
            .add_systems(Startup, spawn_cursor)
            .add_systems(Update, move_cursor);
    }
}

fn create_window() -> Window {
    Window {
        title: "Fred's Revenge".to_string(),
        resolution: (800.0, 600.0).into(),
        resizable: true,
        cursor: Cursor {
            visible: false,
            ..default()
        },
        ..default()
    }
}

fn spawn_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("Cursor.png");
    commands.spawn((
        CustomCursor,
        SpriteSheetBundle {
            texture,
            ..default()
        },
    ));
    commands.insert_resource(CursorDirection(Vec2::ZERO));
}

fn move_cursor(
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut transform_q: Query<&mut Transform, With<CustomCursor>>,
    mut direction: ResMut<CursorDirection>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window_q.single();
    if let Some(vec) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        transform_q.single_mut().translation = vec.extend(5.0);
        direction.0 = vec.normalize();
    }
}
