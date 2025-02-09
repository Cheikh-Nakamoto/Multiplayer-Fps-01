use bevy::prelude::*;
use std::collections::HashMap;
use crate::data::entities::{
    player::Player,
    clients::Client,
    udp::{UDP, UDPMethod},
};


pub fn move_client_system(
    mut query: Query<(&mut Transform, &Player)>,
    keyboard: Res<Input<KeyCode>>,
    client: Res<Client>,
) {
    let velocity = 1.0;

    for (mut transform, _player) in query.iter_mut() {
        let mut position = Vec3::ZERO;
        if keyboard.pressed(KeyCode::W) {
            position.z -= velocity;
        }
        if keyboard.pressed(KeyCode::S) {
            position.z += velocity;
        }
        if keyboard.pressed(KeyCode::A) {
            position.x -= velocity;
        }
        if keyboard.pressed(KeyCode::D) {
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


pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // Joueur
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player::default(),
    ));

    println!("Setup system executed!"); // Debug print
}