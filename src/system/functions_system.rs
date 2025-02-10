use bevy::{pbr::{NotShadowCaster, NotShadowReceiver}, prelude::*};
use std::collections::HashMap;
use crate::data::entities::{
    player::Player,
    clients::Client,
    udp::{UDP, UDPMethod},
};


pub fn move_client_system(
    mut query: Query<(&mut Transform, &Player)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    client: Res<Client>,
) {
    let velocity = 1.0;

    for (mut transform, _player) in query.iter_mut() {
        let mut position = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyW) {
            position.z -= velocity;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            position.z += velocity;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            position.x -= velocity;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            position.x += velocity;
        }
        
        if position != Vec3::ZERO {
            transform.translation += position;
            let mut data = HashMap::new();
            data.insert("type".to_string(), "movement".to_string());
            data.insert("username".to_string(), client.username().clone());
            data.insert("x".to_string(), transform.translation.x.to_string());
            data.insert("y".to_string(), transform.translation.y.to_string());
            data.insert("z".to_string(), transform.translation.z.to_string());

            let server_addr = client.server().clone();
            println!("Trying to send to server: {}", server_addr);
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    match UDP::create_socket_sender(0).await {
                        Ok(socket) => {
                            let json_msg = serde_json::to_string(&data).unwrap();
                            match socket.send(json_msg.clone(), server_addr.clone()).await {
                                Ok(_) => println!("Message sent successfully: {}", json_msg),
                                Err(e) => println!("Failed to send message: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to create socket: {}", e),
                    }
                });
            });
        }
    }
}


