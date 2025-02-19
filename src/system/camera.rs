use bevy::app::Plugin;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::system::Res;
// use bevy::color::Color;
use bevy::math::{ Vec2, Vec3 };
// use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::{ Camera3d, Commands, Startup, Transform };
use bevy::transform::components::GlobalTransform;
// use bevy::render::primitives::Sphere;
use bevy_rapier3d::prelude::{
    ActiveEvents,
    Ccd,
    CoefficientCombineRule,
    Collider,
    ColliderMassProperties,
    Damping,
    Friction,
    GravityScale,
    LockedAxes,
    Restitution,
    RigidBody,
    Velocity,
};

use crate::data::entities::clients::Client;
use crate::data::entities::player::Player;

use super::camera_controller;
use super::collision_detection::{CustomCollider, CustomColliderType};

//const CAMERA_INIT_POS: f64 = 80.0;

pub struct CameraPlugins;

impl Plugin for CameraPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera_player);
    }
}

#[derive(Bundle)]
pub struct CameraPlayerBundle {
    pub camera: Camera3d,
    pub controller: camera_controller::CameraController,
    // Transform et GlobalTransform font déjà partie d'un bundle, on les inclut ici explicitement.
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub ccd: Ccd,
    pub player: Player,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub mass_props: ColliderMassProperties,
    pub gravity_scale: GravityScale,
    pub locked_axes: LockedAxes,
    pub friction: Friction,
    pub restitution: Restitution,
    pub damping: Damping,
    pub velocity: Velocity,
    pub active_events: ActiveEvents,
    pub custom_collider: CustomCollider,
}

fn spawn_camera_player(mut command: Commands, client: Res<Client>) {
    let mut player = Player::new();
    player.username = client.username();
    let camera_object_radius = 4.0;
    // A camera:
    command
        .spawn(CameraPlayerBundle {
            camera: Camera3d::default(),
            controller: camera_controller::CameraController {
                sensitivity: 0.1,
                rotation: Vec2::ZERO,
                rotation_lock: 45.0,
            },
            transform: Transform::from_xyz(24.0, 3.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            global_transform: GlobalTransform::default(),
            ccd: Ccd::enabled(),
            player,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, camera_object_radius),
            mass_props: ColliderMassProperties::Mass(1.0),
            gravity_scale: GravityScale(98.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            friction: Friction {
                coefficient: 5.0,
                combine_rule: CoefficientCombineRule::Max,
            },
            restitution: Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            damping: Damping {
                linear_damping: 8.0,
                angular_damping: 2.0,
            },
            velocity: Velocity::zero(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            custom_collider: CustomCollider::new(camera_object_radius, CustomColliderType::Player),
        });
}
