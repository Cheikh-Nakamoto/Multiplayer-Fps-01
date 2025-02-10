use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use crate::data::entities::{clients::Client, player::Player};
pub struct WorldConigPlugin;

impl Plugin for WorldConigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, World_config);
    }
}

pub fn World_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(20.0, 5.0, 0.25))),
    //     MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
    //     Transform::from_xyz(0.0, 2.0, -10.0),
    // ));

    // Mur Sud
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(20.0, 5.0, 0.25))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 3.0, 10.0),
    ));

    // Mur Est
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.25, 5.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(10.0, 3.0, 0.0),
    ));

    // Mur Ouest
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.25, 5.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(-10.0, 3.0, 0.0),
    ));
}
