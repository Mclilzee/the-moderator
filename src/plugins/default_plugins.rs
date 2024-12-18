use bevy::{
    asset::AssetMetaCheck, prelude::*, window::PrimaryWindow, winit::cursor::{CursorIcon, CustomCursor}
};

pub struct CustomDefaultPlugin;

#[derive(Resource, Default)]
pub struct CursorPosition(pub Vec2);

impl Plugin for CustomDefaultPlugin {
    fn build(&self, app: &mut App) {
        let window = Window {
            title: "The Moderator: Fred's Revenge".to_string(),
            resolution: (1920.0, 1080.0).into(),
            resizable: true,
            ..default()
        };

        let default_plugins = DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(window),
                ..default()
            }).set(AssetPlugin {
                
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .build();

        app.insert_resource(CursorPosition::default())
            .add_plugins(default_plugins)
            .add_systems(PostStartup, insert_cursor)
            .add_systems(Update, update_cursor_posiion);
    }
}

fn insert_cursor(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    asset_server: Res<AssetServer>,
) {
    commands.entity(*window).insert((
        CursorIcon::Custom(CustomCursor::Image {
            handle: asset_server.load("./cursor.png"),
            hotspot: (5, 5),
        }),
        Transform::default(),
    ));
}

fn update_cursor_posiion(
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window_q.single();
    if let Some(v) = window
        .cursor_position()
        .map(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        cursor_position.0 = v;
    }
}
