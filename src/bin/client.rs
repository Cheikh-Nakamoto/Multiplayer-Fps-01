use bevy::prelude::*;
use bevy::utils::HashMap;
use multiplayer_fps::data::entities::clients::{Client, ClientMethods};
use multiplayer_fps::data::entities::player::Player;
use multiplayer_fps::data::entities::udp::{UDPMethod, UdpReceiver, UDP};
use multiplayer_fps::system::camera::CameraPlugins;
use multiplayer_fps::system::camera_controller::update_camera_controller;
use multiplayer_fps::system::functions_system::{control_cursor, move_client_system, setup_mouse};
use multiplayer_fps::system::light::LigthPlugin;
use multiplayer_fps::system::map::WorldConigPlugin;
use tokio::sync::mpsc;
use std::io::Error;
use tokio::runtime::Runtime;
use bevy_rapier3d::prelude::*;

// #[tokio::main]

fn main() -> Result<(), Error> {
    let rt = Runtime::new().unwrap();
    // Créer un canal pour communiquer entre l'asynchrone et Bevy
    let (sender, receiver) = mpsc::channel(32);
    let client = rt.block_on(async {
        let (username, ip_addr) = Client::collect()?;
        let mut client = Client::new(username.clone(), Player::default(), ip_addr.clone());
        client.connect(username, ip_addr).await?;
        Ok::<_, Error>(client)
    })?;

    rt.spawn(async move {
        let udp = match UDP::create_socket_sender(8081).await {
            Ok(udp) => udp,
            Err(e) => {
                eprintln!("Failed to create UDP socket: {}", e);
                return;
            }
        };
    
        loop {
            match udp.receive().await {
                Ok((message, _)) => {
                    // Désérialiser le message en HashMap<String, String>
                    match serde_json::from_str::<HashMap<String, String>>(&message) {
                        Ok(information) => {
                            // Envoyer le HashMap via le canal
                            if sender.send(information).await.is_err() {
                                eprintln!("Failed to send data to Bevy");
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to deserialize message: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                }
            }
        }
    });

    App::new()
        .insert_resource(client)
        .insert_resource(UdpReceiver { receiver })
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
