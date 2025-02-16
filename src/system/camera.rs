use bevy::app::Plugin;
use bevy::ecs::system::Res;
// use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
// use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::{Camera3d, Commands, Startup, Transform};
use bevy::render::camera::Camera;
use bevy::utils::tracing::instrument::WithSubscriber;
// use bevy::render::primitives::Sphere;
use bevy_rapier3d::prelude::{
    ActiveEvents, AdditionalMassProperties, Ccd, CoefficientCombineRule, Collider,
    ColliderMassProperties, Damping, Friction, GravityScale, LockedAxes, Restitution, RigidBody,
    Velocity,
};

use crate::data::entities::clients::Client;
use crate::data::entities::player::Player;

use super::camera_controller;

//const CAMERA_INIT_POS: f64 = 80.0;

pub struct CameraPlugins;

impl Plugin for CameraPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera_player);
    }
}

fn spawn_camera_player(mut command: Commands, client: Res<Client>) {
    let mut player = Player::new();
    player.username = client.username();
    // A camera:
    command.spawn((
        Camera3d::default(),
        camera_controller::CameraController {
            sensitivity: 0.1,
            rotation: Vec2::ZERO,
            rotation_lock: 45.0,
        },
        Transform::from_xyz(24.0, 2.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        Ccd::enabled(),
        player,
        RigidBody::Dynamic,
        Collider::capsule(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 0.5),
        ColliderMassProperties::Mass(1.0),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
        Friction {
            coefficient: 2.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Damping {
            linear_damping: 5.0,
            angular_damping: 1.0,
        },
        Velocity::zero(),
        ActiveEvents::COLLISION_EVENTS,
    ));
}
