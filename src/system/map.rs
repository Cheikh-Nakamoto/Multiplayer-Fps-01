use bevy::{pbr::{NotShadowCaster, NotShadowReceiver}, prelude::*};

use crate::data::entities::{
    player::Player,
    clients::Client,
};
pub struct  WorldConigPlugin;


impl Plugin for WorldConigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,World_config);
    }
}

pub fn World_config(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Player::default(),
    ));

}


