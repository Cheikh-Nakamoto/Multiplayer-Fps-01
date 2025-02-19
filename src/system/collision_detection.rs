use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Nature {
    Bullet,
    Player(String),
    Wall,
    Ground,
    Sky
}

#[derive(Component, Debug, Clone)]
pub struct CustomCollider {
    pub radius: f32,
    pub colliding_entities: Vec<(Entity, Nature)>,
    pub nature: Nature,
}

impl CustomCollider {
    pub fn new(radius: f32, nature: Nature) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
            nature,
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (collisions_detection, handle_collisions));
    }
}

/// Detects collisions between entities based on their positions and collider radii.
///
/// # Arguments
///
/// - `query`: A mutable reference to a `Query` containing entities with `GlobalTransform` and `CustomCollider` components.
///
/// # Description
///
/// This function iterates over the entities in the `query` and checks for collisions between each pair of entities. If a collision is detected, the colliding entities are stored in a `HashMap` called `colliding_entities`.
///
/// The collision detection is performed by calculating the distance between the translation of each pair of entities and comparing it to the sum of their respective collider radii. If the distance is less than the sum of the radii, a collision is considered to have occurred.
///
/// After all collisions have been detected, the function updates the `colliding_entities` field of each `CustomCollider` component in the `query` based on the detected collisions.
///
/// # Example
///
/// ```rust
/// fn main() {
///     // Create entities and add necessary components
///     let mut world = World::default();
///     let mut schedule = Schedule::default();
///
///     // Add systems to the schedule
///     schedule.add_system(collisions_detection.system());
///
///     // Run the schedule
///     schedule.run(&mut world);
/// }
/// ```
///
/// # Note
///
/// This function assumes that the `CustomCollider` component has the following fields:
/// - `radius`: The radius of the collider.
/// - `colliding_entities`: A vector of tuples containing the colliding entity and its nature.
///
/// The `GlobalTransform` component is assumed to provide the translation information of each entity.
///
fn collisions_detection(mut query: Query<(Entity, &GlobalTransform, &mut CustomCollider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<(Entity, Nature)>> = HashMap::new();

    // Detect collisions
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a.translation().distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push((entity_b, collider_b.nature.clone()));
                }
            }
        }
    }

    // Update colliders
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.colliding_entities.extend(collisions.iter().cloned());
        }
    }
}

/// Handles collisions by setting the linear velocity to zero for entities that are colliding.
pub fn handle_collisions(
    mut query: Query<(&CustomCollider, &mut Velocity)>,
    // Retirer With<MapBlock>
) {
    for (collider, mut velocity) in query.iter_mut() {
        if !collider.colliding_entities.is_empty() {
            velocity.linvel = Vec3::ZERO;
        }
    }
}
