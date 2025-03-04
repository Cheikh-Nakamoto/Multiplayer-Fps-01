use bevy::prelude::*;
use bevy::utils::HashMap;
use multiplayer_fps::data::entities::clients::*;
use multiplayer_fps::data::entities::player::Player;
use multiplayer_fps::data::entities::udp::*;
use multiplayer_fps::system::camera::CameraPlugins;
use multiplayer_fps::system::camera_controller::*;
use multiplayer_fps::system::collision_detection::*;
use multiplayer_fps::system::fps_tool::OverlayColorPlugin;
use multiplayer_fps::system::functions_system::*;
use multiplayer_fps::system::light::LigthPlugin;
use multiplayer_fps::system::map::WorldConigPlugin;
use multiplayer_fps::system::minimap::MinimapPlugin;
use multiplayer_fps::system::receiver_server::ReceiverPlugin;
use multiplayer_fps::system::shoot_player::TracerPlugin;
use tokio::sync::mpsc;
use std::io::Error;
use tokio::runtime::Runtime;
use bevy_rapier3d::prelude::*;

// #[tokio::main]

fn main() -> Result<(), Error> {
    let mut user = String::new();
    let rt = Runtime::new().unwrap();
    // Créer un canal pour communiquer entre l'asynchrone et Bevy
    let (sender, receiver) = mpsc::channel(32);
    let client = rt.block_on(async {
        let (username, ip_addr) = Client::collect()?;
        user = username.clone();
        let mut client = Client::new(username.clone(), Player::default(), ip_addr.clone());
        client.connect(username, ip_addr).await?;
        Ok::<_, Error>(client)
    })?;
    let clone_client = client.clone();
    rt.spawn(async move {
        let udp = match UDP::create_socket_sender(8081).await {
            Ok(udp) => udp,
            Err(e) => {
                eprintln!("Failed to create UDP socket: {}", e);
                return;
            }
        };
        let mut hash = HashMap::new();
        hash.insert("type", "participants");
        hash.insert("username", user.as_str());
        let hash_to_str = serde_json::to_string(&hash).unwrap_or_default();
        let _ = udp.send(hash_to_str, client.server()).await;
        loop {
            match udp.receive().await {
                Ok((message, _)) => {
                    // Désérialiser le message en HashMap<String, String>
                    match serde_json::from_str::<HashMap<String, String>>(&message) {
                        Ok(information) => {
                            println!("data .. {:?}", information);
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

    let mut render_plugin = RapierDebugRenderPlugin::default();
    render_plugin.default_collider_debug = ColliderDebug::NeverRender;

    App::new()
        .insert_resource(clone_client)
        .insert_resource(UdpReceiver { receiver })
        .insert_resource(AmbientLight {
            brightness: 100.0,
            ..default()
        })
        .add_systems(Startup, setup_mouse)
        .add_plugins((
            DefaultPlugins,
            CameraPlugins,
            LigthPlugin,
            WorldConigPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            render_plugin,
            ReceiverPlugin,
            CollisionDetectionPlugin,
            TracerPlugin,
            OverlayColorPlugin,
            MinimapPlugin
        ))
        .add_systems(Update, (
            move_client_system,
            update_camera_controller,
            control_cursor,
            handle_collisions,
        ))
        .run();

    Ok(())
}
