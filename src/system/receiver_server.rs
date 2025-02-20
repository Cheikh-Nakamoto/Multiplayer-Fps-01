use bevy::{
    app::{Plugin, Update},
    asset::Assets,
    color::Color,
    ecs::{
        entity::Entity,
        system::{Commands, Query, ResMut},
    },
    math::{primitives::Cuboid, Vec3},
    pbr::{MeshMaterial3d, StandardMaterial},
    render::mesh::{Mesh, Mesh3d},
    transform::components::Transform,
    utils::{default, HashMap},
};
use bevy_rapier3d::prelude:: RigidBody;

use crate::{
    data::entities::{player::Player, udp::UdpReceiver},
    utils::get_field::{get_field, get_pos_player},
};

use super::collision_detection::{CustomCollider, Nature};

pub struct ReceiverPlugin;

impl Plugin for ReceiverPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, receiver_data);
    }
}

fn receiver_data(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut udp_receiver: ResMut<UdpReceiver>,
    mut player_query: Query<(Entity, &mut Transform, &mut Player)>, // Ajout de la requête ici
) {
    // Lire les

    while let Ok(information) = udp_receiver.receiver.try_recv() {
        // Extraire le type de message
        let type_msg = get_field(information.clone(), "type");
        println!("Received data: {:?}", information);
        match type_msg.as_str() {
            "join" => {
                dbg!("<====================================================>\n\n");
                dbg!("Player joined: {:?}\n", &information);
                dbg!("<====================================================>\n\n");

                // Traiter l'événement "join"
                // Extraire le nom du joueur et sa position
                let username = get_field(information, "username");

                // Créer l'entité du joueur
                dbg!("===============================preparing add other player======================================================================");

                spawn_other_player(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    username,
                    Vec3::new(24.0, 2.5, 0.0),
                );
            }
            "movement" => {
                // Traiter l'événement "move"
                if let Some(username) = information.get("username") {
                    println!("<======================================>");
                    println!("<===========Move player update: {}============>", username);
                    // Convertir la position en Vec3
                    let new_position = get_pos_player(information.clone());
                    // Mettre à jour la position du joueur
                    for (_, transform, player) in player_query.iter_mut() {
                        if player.username == username.trim() {
                            let _ = transform.translation.lerp(new_position, 0.1);
                            println!(
                                "<===========Movement update successfully: {:?}============>",
                                new_position
                            );
                            break;
                        }
                    }
                    println!("<===============finish================>");
                }
            }
            "disconnection" => {
                println!("Player left: {:?}", information);
                // Traiter l'événement "leave"
                if let Some(username) = information.get("username") {
                    // Supprimer l'entité du joueur
                    for (entity, _, player) in player_query.iter() {
                        if player.username == username.trim() {
                            commands.entity(entity).despawn();
                        }
                    }
                }
            }
            "participants" => {
                dbg!("<====================================================>\n\n");
                dbg!("Player participitants: {:?}\n", &information);
                dbg!("<====================================================>\n\n");
                // Traiter l'événement "leave"
                let data = deserialize_player_positions(information);
                for (cle, pos) in data {
                    if &cle == "type" || &cle == "statut" {
                        continue;
                    }
                    spawn_other_player(&mut commands, &mut meshes, &mut materials, cle, pos);
                }
            }
            _ => {
                println!("Unknown message type: {}", type_msg);
            }
        }
    }
}

fn spawn_other_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    username: String,
    position: Vec3,
) {
    let mut player = Player::new();
    player.username = username.clone();
    // Création du cube pour représenter le joueur
    let player_mesh = meshes.add(Cuboid::new(2.0, 1.5, 2.0)); // Mesh pour un cube
    let player_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1., 0., 0.), // Couleur bleue
        ..default()
    });
    // Créer l'entité du joueur
    commands.spawn((
        RigidBody::Dynamic,
        Mesh3d(player_mesh),
        MeshMaterial3d(player_material),
        Transform::from_xyz(position.x, 2.5, position.z).looking_at(Vec3::ZERO, Vec3::Y),
        CustomCollider::new(1.0, Nature::Player(username)),
        player,
    ));
}

fn deserialize_player_positions(player_map: HashMap<String, String>) -> HashMap<String, Vec3> {
    let mut result: HashMap<String, Vec3> = HashMap::new();

    for (username, json_position) in player_map {
        if username == "type" || username == "status" {
            continue; // Ignore l'entrée "type"
        }

        match serde_json::from_str::<Vec3>(&json_position) {
            Ok(position) => {
                result.insert(username, position);
            }
            Err(e) => {
                eprintln!("Erreur de désérialisation pour {}: {}", username, e);
            }
        }
    }

    result
}
