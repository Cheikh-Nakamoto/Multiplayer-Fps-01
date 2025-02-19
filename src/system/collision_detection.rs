use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum CustomColliderType {
    Player,
    Obstacle,
    Bullet,
}

#[derive(Component, Debug, Clone)]
pub struct CustomCollider {
    pub radius: f32,
    pub collider_type: CustomColliderType,
    pub colliding_entities: Vec<Hitted>,
}

#[derive(Debug, Clone)]
pub struct Hitted {
    pub entity: Entity,
    pub collider: CustomCollider,
}

impl Hitted {
    pub fn new(entity: Entity, collider: CustomCollider) -> Self {
        Self { entity, collider }
    }
}

impl CustomCollider {
    pub fn new(radius: f32, collider_type: CustomColliderType) -> Self {
        Self {
            radius,
            collider_type,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (collisions_detection, handle_collisions));
    }
}

/// Performs collision detection between entities with colliders.
///
/// # Arguments
///
/// - `query`: A mutable query containing entities with `GlobalTransform` and `CustomCollider` components.
///
/// # Description
///
/// This function detects collisions between entities by iterating over the provided query. It calculates the distance between the entities' translations and checks if it is less than the sum of their radii. If a collision is detected, the colliding entities are stored in a `HashMap` where the key is the entity that collided and the value is a vector of entities it collided with.
///
/// After detecting all collisions, the function updates the colliders by clearing the `colliding_entities` field and adding the corresponding colliding entities for each entity in the query.
///
/// Note: The distance calculation assumes that the entities' translations are represented by vectors.
///
/// # Example
///
/// ```
/// fn main() {
///     // Create entities with colliders and transformations
///     let mut query = Query::<(Entity, &GlobalTransform, &mut CustomCollider)>::new();
///     query.push((entity_1, transform_1, collider_1));
///     query.push((entity_2, transform_2, collider_2));
///
///     // Perform collision detection
///     collisions_detection(query);
/// }
/// ```
fn collisions_detection(mut query: Query<(Entity, &GlobalTransform, &mut CustomCollider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Hitted>> = HashMap::new();

    // Detect collisions
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                if distance + 2.0 < collider_a.radius + collider_b.radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(Hitted::new(entity_b, collider_b.clone()));
                }
            }
        }
    }

    // Update colliders
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().cloned());
        }
    }
}

pub fn handle_collisions(
    mut command: Commands,
    mut query: Query<(&CustomCollider, &mut Velocity)>,
    // Retirer With<MapBlock>
) {
    for (collider, mut velocity) in query.iter_mut() {
        if !collider.colliding_entities.is_empty() {
            // if collider.collider_type != CustomColliderType::Bullet {
            //     velocity.linvel = Vec3::ZERO;
            // } else {
                
            //     command.entity(collider.colliding_entities[0]).despawn();
            // }
            for el in &collider.colliding_entities {
                if collider.collider_type != CustomColliderType::Bullet && el.collider.collider_type == CustomColliderType::Bullet {
                    command.entity(el.entity).despawn();
                } else {
                    velocity.linvel = Vec3::ZERO;
                }
            }
        }
    }
}
