use bevy::prelude::*;
use multiplayer_fps::data::entities::clients::{Client, ClientMethods};
use multiplayer_fps::data::entities::player::Player;
use multiplayer_fps::system::camera::CameraPlugins;
use multiplayer_fps::system::camera_controller::update_camera_controller;
use multiplayer_fps::system::functions_system::{control_cursor, move_client_system, setup_mouse};
use multiplayer_fps::system::light::LigthPlugin;
use multiplayer_fps::system::map::WorldConigPlugin;
use std::io::Error;
use tokio::runtime::Runtime;
use bevy_rapier3d::prelude::*;

// #[tokio::main]
fn main() -> Result<(), Error> {
    let rt = Runtime::new().unwrap();

    let client = rt.block_on(async {
        let (username, ip_addr) = Client::collect()?;
        let mut client = Client::new(username.clone(), Player::default(), ip_addr.clone());
        client.connect(username, ip_addr).await?;
        Ok::<_, Error>(client)
    })?;

    App::new()
        .insert_resource(client)
        .insert_resource(AmbientLight {
            brightness: 100.0,
            ..default()
        })
        .add_systems(Startup, setup_mouse)
        .add_plugins((DefaultPlugins,CameraPlugins,LigthPlugin,WorldConigPlugin, RapierPhysicsPlugin::<NoUserData>::default(), RapierDebugRenderPlugin::default(),           
        ))
        .add_systems(
            Update,
            (move_client_system, update_camera_controller, control_cursor),
        )
        .run();

    Ok(())
}
