use bevy::prelude::*;
use bevy_rapier3d::prelude::ActiveEvents;
use crate::data::entities::{ clients::Client, player::Player };

use super::collision_detection::{ CustomCollider, Nature };
pub struct TracerPlugin;
impl Plugin for TracerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_tracers, shoot));
    }
}

// Composant pour représenter un projectile
#[derive(Component)]
pub struct BulletTracer {
    pub start_position: Vec3, // Point de départ du projectile
    pub end_position: Vec3, // Point d'arrivée du projectile
    pub lifetime: f32, // Durée de vie totale
    pub time_alive: f32, // Temps écoulé depuis la création
}

impl BulletTracer {
    // Crée un nouveau projectile
    pub fn new(start: Vec3, end: Vec3, speed: f32) -> BulletTracer {
        BulletTracer {
            start_position: start,
            end_position: end,
            lifetime: Vec3::distance(start, end) / speed, // Calcule la durée de vie basée sur la distance et la vitesse
            time_alive: 0.0,
        }
    }
}

// Système de tir
fn shoot(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&Transform, &Player), With<Camera3d>>,
    mut meshes: ResMut<Assets<Mesh>>,
    client: Res<Client>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Vérifie si la touche espace est pressée
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok((player_transform, player)) = query.get_single() {
            // Vérifie si c'est le bon joueur
            if client.username() == player.username {
                let offset_distance = 0.1;
                let shoot_direction = player_transform.forward(); // Direction du tir
                let start_position =
                    player_transform.translation + shoot_direction * offset_distance; // Position de départ
                let end_position = start_position + shoot_direction * 50.0; // Position d'arrivée
                let bullet_speed = 500.0; // Vitesse du projectile
                let base_color = Color::srgb(0.9, 0.2, 0.3); // Couleur rouge

                // Crée le projectile avec tous ses composants
                commands.spawn((
                    // Collider::ball(0.1),
                    ActiveEvents::COLLISION_EVENTS,
                    BulletTracer::new(start_position, end_position, bullet_speed),
                    Mesh3d(meshes.add(Sphere::new(0.05).mesh().ico(7).unwrap())),
                    MeshMaterial3d(
                        materials.add(StandardMaterial {
                            base_color,
                            alpha_mode: AlphaMode::Opaque,
                            ..default()
                        })
                    ),
                    CustomCollider::new(0.07, Nature::Bullet),
                ));
            }
        }
    }
}

// Système de mise à jour des projectiles
fn update_tracers(
    mut commands: Commands,
    mut tracer_query: Query<(&mut BulletTracer, &mut Transform, Entity, &mut CustomCollider)>,
    time: Res<Time>,
    client: Res<Client>
) {
    // Pour chaque projectile existant
    for (mut tracer, mut transform, entity, mut collider) in tracer_query.iter_mut() {
        tracer.time_alive += time.delta_secs(); // Met à jour le temps de vie

        // Calcule la nouvelle position avec une interpolation linéaire
        transform.translation = Vec3::lerp(
            tracer.start_position,
            tracer.end_position,
            f32::clamp(tracer.time_alive / tracer.lifetime, 0.0, 1.0)
        );

        // Oriente le projectile vers sa destination
        transform.look_at(tracer.end_position, Vec3::Y);

        if !collider.colliding_entities.is_empty() {
            let is_ennemy = match collider.colliding_entities[0].1.clone() {
                Nature::Player(s) if s != client.username() => true,
                _ => false,
            };
    
            let target_hitted =
            collider.nature == Nature::Bullet &&
            collider.colliding_entities[0].1 == Nature::Wall;
    
            // Supprime le projectile si sa durée de vie est dépassée
            // ou si une balle atteint un obstacle
            if tracer.time_alive > tracer.lifetime || target_hitted {
                commands.entity(entity).despawn();
            }

            // Supprime le projectile et l'ennemi lorsque ce dernier est touché
            if is_ennemy {
                commands.entity(entity).despawn();
                commands.entity(collider.colliding_entities[0].0).despawn();
            }
            collider.colliding_entities.clear();
        }
    }
}
