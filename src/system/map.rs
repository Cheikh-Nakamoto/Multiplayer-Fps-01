use std::{f32::consts::PI, path::Path};

use bevy::{
    asset::{io::AssetSourceId, AssetPath},
    pbr::{CascadeShadowConfigBuilder, NotShadowCaster},
    prelude::*,
};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction, Restitution, RigidBody};

pub struct WorldConigPlugin;

impl Plugin for WorldConigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, world_config);
    }
}

pub fn world_config(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50., 50.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Collider::cuboid(25.0, 0.1, 25.0), // Collision pour le sol
        RigidBody::Fixed,                  // Le sol ne doit pas bouger
    ));

    let wall_color1 = materials.add(Color::srgb(0.65, 0.32, 0.17));
    let wall_color2 = materials.add(Color::srgb(0.55, 0.27, 0.07));

    let walls = [
        (0.0, 2.5, 25.0, 50.0, 5.0, 0.5),  // Mur Nord
        (0.0, 2.5, -25.0, 50.0, 5.0, 0.5), // Mur Sud
        (25.0, 2.5, 0.0, 0.5, 5.0, 50.0),  // Mur Est
        (-25.0, 2.5, 0.0, 0.5, 5.0, 50.0), // Mur Ouest
    ];
    let mut collider_compound = vec![];
    for &(x, y, z, w, h, d) in &walls {
        collider_compound.push((
            Vec3::new(x, y, z),                          // Position relative
            Quat::IDENTITY,                              // Pas de rotation
            Collider::cuboid(w / 2.0, h / 2.0, d / 2.0), // Dimensions (w/2, h/2, d)
        ));
    }
    for &(x, y, z, w, h, d) in &walls {
        commands.spawn((
            RigidBody::Fixed,
            Collider::cuboid(w/2., h/2.0, d/2.),
            Friction {
                coefficient: 2.,
                    combine_rule: CoefficientCombineRule::Max,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Transform::from_xyz(x, y, z),
            Mesh3d(meshes.add(Cuboid::new(w, h, d))),
            MeshMaterial3d(wall_color1.clone()),
        ));
    }
    commands.spawn((
        RigidBody::Fixed,
        Collider::compound(collider_compound.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    let maze_walls = [
        (-10.0, 2.5, -10.0, 5.0, 2.0, 0.75),
        (10.0, 2.5, 10.0, 5.0, 2.0, 0.75),
        (-10.0, 2.5, 10.0, 5.0, 2.0, 0.75),
        (10.0, 2.5, -10.0, 5.0, 2.0, 0.75),
    ];
    let wall_intersection_pair = vec![
        (-10.0, 2.5, -5.0, 20.0, 5.0, 0.75),
        (-15.0, 2.5, 0.0, 0.75, 5.0, 20.0),
        (15.0, 2.5, -6.0, 0.75, 5.0, 22.0),
        (10.0, 2.5, 5.0, 20.0, 5.0, 0.75),
        (1.0, 2.5, 20.0, 35.0, 5.0, 0.75),
        (-5.0, 2.5, 10.0, 0.75, 5.0, 20.0),
        (-1.0, 2.5, -20.0, 35.0, 5.0, 0.75),
        (5.0, 2.5, -10.0, 0.75, 5.0, 25.0),
    ];
    let mut paire_hit = Vec::new();
    let mut wall_paire = Vec::new();

    for chunk in wall_intersection_pair.chunks_exact(2) {
        for &(x, y, z, w, h, d) in chunk {
            paire_hit.push((
                Vec3::new(x, y, z),                          // Position relative
                Quat::IDENTITY,                              // Pas de rotation
                Collider::cuboid(w / 2.0, h / 2.0, d / 2.0), // Dimensions (w/2, h/2, d/2)
            ));
            wall_paire.push((x, y, z, w, h, d));
        }

        commands.spawn((
            RigidBody::Fixed,
            Collider::compound(paire_hit.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
        // Créer les entités pour le rendu visuel des murs
        for &(x, y, z, w, h, d) in &wall_paire {
            commands.spawn((
                Transform::from_xyz(x, y, z),
                Mesh3d(meshes.add(Cuboid::new(w, h, d))),
                MeshMaterial3d(wall_color1.clone()),
            ));
        }
        paire_hit.clear();
        wall_paire.clear();
    }
    for &(x, y, z, w, h, d) in &maze_walls {
        commands.spawn((
            RigidBody::Fixed,
            Collider::cuboid(w/2.0, h/2.0, d/2.0),
            Friction {
                coefficient: 2.,
                    combine_rule: CoefficientCombineRule::Max,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Mesh3d(meshes.add(Cuboid::new(w, h, d))),
            MeshMaterial3d(wall_color2.clone()),
            Transform::from_xyz(x, y, z),
        ));
    }
    //Sky
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(50.0, 30.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("888888").unwrap().into(),
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform::from_scale(Vec3::splat(30.0)),
        NotShadowCaster,
    ));
}
