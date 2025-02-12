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
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let path = Path::new("assets/textures/space.jpg");
    let source = AssetSourceId::from("wall");
    let asset_path = AssetPath::from_path(path).with_source(source);
    commands.spawn((
        
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50., 50.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    let wall_color1 = materials.add(Color::srgb(0.65, 0.32, 0.17));
    let wall_color2 = materials.add(Color::srgb(0.55, 0.27, 0.07));

    // let origin = [25.0, 0.0, 25.0];

    let walls = [
        (0.0, 2.5, 25.0, 50.0, 5.0, 0.25),  // Mur Nord
        (0.0, 2.5, -25.0, 50.0, 5.0, 0.25), // Mur Sud
        (25.0, 2.5, 0.0, 0.25, 5.0, 50.0),  // Mur Est
        (-25.0, 2.5, 0.0, 0.25, 5.0, 50.0), // Mur Ouest
    ];

    for &(x, y, z, w, h, d) in &walls {
        commands.spawn((
            RigidBody::Fixed,
            Collider::cuboid(w/2.0, h/2.0, d/2.0),
            Friction {
                coefficient: 0.7,
                combine_rule: CoefficientCombineRule::Min,
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

    let maze_walls = [
        (-1.0, 2.5, -20.0, 35.0, 5.0, 0.25),
        (1.0, 2.5, 20.0, 35.0, 5.0, 0.25),
        (-15.0, 2.5, 0.0, 0.25, 5.0, 20.0),
        (15.0, 2.5, -6.0, 0.25, 5.0, 23.0),
        (-10.0, 2.5, -5.0, 20.0, 5.0, 0.25),
        (10.0, 2.5, 5.0, 20.0, 5.0, 0.25),
        (-5.0, 2.5, 10.0, 0.25, 5.0, 20.0),
        (5.0, 2.5, -10.0, 0.25, 5.0, 25.0),
        (-10.0, 2.5, -10.0, 5.0, 2.0, 0.25),
        (10.0, 2.5, 10.0, 5.0, 2.0, 0.25),
        (-10.0, 2.5, 10.0, 5.0, 2.0, 0.25),
        (10.0, 2.5, -10.0, 5.0, 2.0, 0.25),
    ];

    for &(x, y, z, w, h, d) in &maze_walls {
        commands.spawn((
            RigidBody::Fixed,
            Collider::cuboid(w/2.0, h/2.0, d/2.0),
            Mesh3d(meshes.add(Cuboid::new(w, h, d))),
            MeshMaterial3d(wall_color2.clone()),
            Transform::from_xyz(x, y, z),
        ));
    }
    let mut transform = Transform::from_xyz(-9.0, 2.5, -15.0);
    transform.rotate_y(-PI / 4.);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(15.0, 3.0, 0.25))),
        MeshMaterial3d(wall_color2.clone()),
        transform,
    ));
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
