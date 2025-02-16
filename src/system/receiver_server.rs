use bevy::{
    app::{Plugin, Update},
    ecs::system::{Commands, Res, ResMut},
    math::Vec3,
    transform::components::{GlobalTransform, Transform},
};
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, Collider, Damping, Friction, GravityScale, LockedAxes, Restitution,
    RigidBody, Velocity,
};

use crate::data::entities::{
    player::Player,
    udp::{UdpReceiver, UDP},
};

pub struct ReceiverPlugin;

impl Plugin for ReceiverPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, receiver_data);
    }
}

fn receiver_data(mut commands: Commands, mut udp_receiver: ResMut<UdpReceiver>) {
    // Lire les données du canal
    while let Ok(information) = udp_receiver.receiver.try_recv() {
        println!("Received data: {:?}", information);

        // Extraire le type de message
        if let Some(type_msg) = information.get("type") {
            match type_msg.as_str() {
                "join" => {
                    println!("<====================================================>\n\n");
                    println!("Player joined: {:?}\n", information);
                    println!("<====================================================>\n\n");

                    // Traiter l'événement "join"
                    // Extraire le nom du joueur et sa position
                    let username = information
                        .get("username")
                        .unwrap_or(&"Unknown".to_string())
                        .clone();
                    let x = information
                        .get("x")
                        .and_then(|v| v.parse::<f32>().ok())
                        .unwrap_or(0.0);
                    let y = information
                        .get("y")
                        .and_then(|v| v.parse::<f32>().ok())
                        .unwrap_or(0.0);
                    let z = information
                        .get("z")
                        .and_then(|v| v.parse::<f32>().ok())
                        .unwrap_or(0.0);

                    // Créer l'entité du joueur
                    spawn_other_player(&mut commands, username, Vec3::new(x, y, z));
                }
                "movement" => {
                    println!("Player moved: {:?}", information);
                    // Traiter l'événement "move"
                }
                "disconnection" => {
                    println!("Player left: {:?}", information);
                    // Traiter l'événement "leave"
                }
                _ => {
                    println!("Unknown message type: {}", type_msg);
                }
            }
        } else {
            println!("Received message without 'type' field: {:?}", information);
        }
    }
}

fn spawn_other_player(
    commands: &mut Commands,
    username: String, // Nom du joueur
    position: Vec3,   // Position initiale du joueur
) {
    let mut player = Player::new();
    player.username = username;
    commands.spawn((
        // Composants visuels et de transformation
        Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        // Composant personnalisé pour le joueur
        player,
        // Composants physiques
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.5, 2.5), // Taille du collider
        GravityScale(0.0),               // Désactiver la gravité
        LockedAxes::ROTATION_LOCKED,     // Verrouiller la rotation
        // Propriétés physiques
        Friction {
            coefficient: 0.0,
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
        Velocity::zero(), // Vitesse initiale nulle
    ));
}
