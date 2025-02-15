use bevy::app::Plugin;
// use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
// use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::{Camera3d, Commands, Startup, Transform};
use bevy::utils::tracing::instrument::WithSubscriber;
// use bevy::render::primitives::Sphere;
use bevy_rapier3d::prelude::{
    ActiveEvents, AdditionalMassProperties, Ccd, CoefficientCombineRule, Collider,
    ColliderMassProperties, Damping, Friction, GravityScale, LockedAxes, Restitution, RigidBody,
    Velocity,
};

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
            sensitivity: 0.1,
            rotation: Vec2::ZERO,
            rotation_lock: 45.0,
        },
        Transform::from_xyz(24.0, 2.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        // DistanceFog {
        //     color: Color::srgba(0.35, 0.48, 0.66, 1.0),
        //     directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.5),
        //     directional_light_exponent: 30.0,
        //     falloff: FogFalloff::from_visibility_colors(
        //         15.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
        //         Color::srgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
        //         Color::srgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
        //     ),
        // },
        Player::new(),
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
        Ccd::enabled(),
        ActiveEvents::COLLISION_EVENTS,
    ));
}
