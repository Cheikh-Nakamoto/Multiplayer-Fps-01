use bevy::{
    app::{Plugin, Update},
    asset::Assets,
    color::Color,
    ecs::{
        entity::Entity,
        system::{Commands, Query, ResMut},
    },
    math::primitives::Cuboid,
    pbr::{MeshMaterial3d, StandardMaterial},
    render::mesh::{Mesh, Mesh3d},
    transform::components::Transform,
    utils::default,
};

use crate::{
    data::entities::{player::Player, udp::UdpReceiver},
    utils::get_field::{get_field, get_pos_player},
};

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
    mut player_query: Query<(Entity, &mut Transform, &Player)>, // Ajout de la requête ici
) {
    // Lire les données du canal
    while let Ok(information) = udp_receiver.receiver.try_recv() {
        println!("Received data: {:?}", information);
        // Extraire le type de message
        let type_msg = get_field(information.clone(),"type"); 
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

                spawn_other_player(&mut commands, &mut meshes, &mut materials, username);
            }
            "movement" => {
                // Traiter l'événement "move"
                if let Some(username) = information.get("username") {
                    println!("<======================================>");
                    println!("<===========Move player update: {}============>", username);
                    // Convertir la position en Vec3
                    let new_position = get_pos_player(information.clone());
                    // Mettre à jour la position du joueur
                    for (_, mut transform, player) in player_query.iter_mut() {
                        if player.username == username.trim() {
                            transform.translation = new_position;
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
                        if player.username == *username {
                            commands.entity(entity).despawn();
                        }
                    }
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
) {
    dbg!("===============================add other player======================================================================");
    let mut player = Player::new();
    player.username = username;
    // Création du cube pour représenter le joueur
    let player_mesh = meshes.add(Cuboid::new(2.0, 1.5, 2.0)); // Mesh pour un cube
    let player_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1., 0., 0.), // Couleur bleue
        ..default()
    });
    // Créer l'entité du joueur
    commands.spawn((Mesh3d(player_mesh), MeshMaterial3d(player_material), player));
}
