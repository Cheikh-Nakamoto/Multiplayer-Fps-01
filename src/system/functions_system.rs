use crate::data::entities::{
    clients::Client,
    player::{self, Player},
    udp::{UDPMethod, UDP},
};
use bevy::window::CursorGrabMode;

use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    time,
};
use std::collections::HashMap;

pub fn move_client_system(
    mut query: Query<(&mut Transform, &Player)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    client: Res<Client>,
    time: Res<Time>,
) {
    let velocity = 1.0;

    for (mut transform, player) in query.iter_mut() {
        let mut movement_factor = 0.0;

        if keyboard.pressed(KeyCode::ArrowUp) {
            movement_factor += velocity;
        }

        let movement_direction = transform.rotation * Vec3::Z;
        let movement_distance = movement_factor * player.movement_speed * time.delta_secs();
        let translation_delta = movement_direction * movement_distance;

        if translation_delta != Vec3::ZERO {
            transform.translation.z -= translation_delta.z;
            transform.translation.x -= translation_delta.x;
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

pub fn setup_mouse(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
}

pub fn control_cursor(mut windows: Query<&mut Window>, keyboard: Res<ButtonInput<KeyCode>>) {
    if let Ok(mut window) = windows.get_single_mut() {
        if keyboard.just_pressed(KeyCode::Escape) {
            window.cursor_options.grab_mode = match window.cursor_options.grab_mode {
                CursorGrabMode::None => CursorGrabMode::Locked,
                CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
            };
            window.cursor_options.visible = !window.cursor_options.visible;
        }
    }
}
