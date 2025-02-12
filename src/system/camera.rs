use bevy::app::Plugin;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Camera3d, Commands, Startup, Transform};

use crate::data::entities::player::Player;

use super::camera_controller;


//const CAMERA_INIT_POS: f64 = 80.0;

pub struct CameraPlugins;

impl Plugin for CameraPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera_player);
    }
}

fn spawn_camera_player(mut command: Commands) {
    // A camera:
    command.spawn((
        Camera3d::default(),
        camera_controller::CameraController {
            sensitivity: 0.035,
            rotation: Vec2::ZERO,
            rotation_lock: 45.0,
        },
        Transform::from_xyz(24.0, 2.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        Player::new()
    ));
}
