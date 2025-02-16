use bevy::{ prelude::*, utils::HashMap };
use bevy_rapier3d::prelude::Velocity;

use super::map_gen::MapBlock;

#[derive(Component, Debug)]
pub struct CustomCollider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl CustomCollider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
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
///     collision_detection(query);
/// }
/// ```
fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut CustomCollider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // Detect collisions
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a.translation().distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    colliding_entities.entry(entity_a).or_insert_with(Vec::new).push(entity_b);
                }
            }
        }
    }

    // Update colliders
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.colliding_entities.extend(collisions.iter().copied());
        }
    }
}

pub fn handle_collisions(mut _commands: Commands, mut query: Query<(Entity, &CustomCollider, &mut Velocity), With<MapBlock>>) {
    for (_, collider, mut velocity) in query.iter_mut() {
        if !collider.colliding_entities.is_empty() {
            velocity.linvel = Vec3::ZERO;
        }
    }
}