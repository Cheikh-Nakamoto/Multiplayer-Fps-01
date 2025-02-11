use std::f32::consts::PI;

use bevy::prelude::*;

pub struct WorldConigPlugin;

impl Plugin for WorldConigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, World_config);
    }
}

pub fn World_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50., 50.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    let wall_color = materials.add(Color::srgb(0.8, 0.7, 0.6));

    let walls = [
        (0.0, 2.5, -25.0, 50.0, 5.0, 0.25), // Mur Nord
        (0.0, 2.5, 25.0, 50.0, 5.0, 0.25),  // Mur Sud
        (25.0, 2.5, 0.0, 0.25, 5.0, 50.0),  // Mur Est
        (-25.0, 2.5, 0.0, 0.25, 5.0, 50.0), // Mur Ouest
    ];

    for &(x, y, z, w, h, d) in &walls {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(w, h, d))),
            MeshMaterial3d(wall_color.clone()),
            Transform::from_xyz(x, y, z),
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
            Mesh3d(meshes.add(Cuboid::new(w, h, d))),
            MeshMaterial3d(wall_color.clone()),
            Transform::from_xyz(x, y, z),
        ));
    }
    let mut transform = Transform::from_xyz(-9.0, 2.5, -15.0);
    transform.rotate_y(-PI / 4.);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(15.0, 3.0, 0.25))),
        MeshMaterial3d(wall_color.clone()),
        transform,
    ));
}
