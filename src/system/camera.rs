use bevy::app::Plugin;
use bevy::math::Vec3;
use bevy::prelude::{Camera3d, Commands, Startup, Transform};


//const CAMERA_INIT_POS: f64 = 80.0;

pub struct CameraPlugins;

impl Plugin for CameraPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut command: Commands) {
    // A camera:
    command.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 15.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
