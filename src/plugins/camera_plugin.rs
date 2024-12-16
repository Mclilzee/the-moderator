use bevy::{prelude::*, render::camera::ScalingMode};

pub const CAMERA_SCALING_WIDTH: f32 = 500.0;
pub const CAMERA_SCALING_HEIGHT: f32 = 400.0;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: CAMERA_SCALING_WIDTH,
                min_height: CAMERA_SCALING_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
