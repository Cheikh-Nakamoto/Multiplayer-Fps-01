use bevy::{
    pbr::NotShadowCaster,
    prelude::*,
};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction, Restitution, RigidBody};

use super::{collision_detection::CustomCollider, map_gen::gen_map};

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
    let cell_width = 10.0;
    // let path = Path::new("assets/textures/space.jpg");
    // let source = AssetSourceId::from("wall");
    // let asset_path = AssetPath::from_path(path).with_source(source);
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50. * cell_width, 50. * cell_width))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Collider::cuboid(50.0, 0.8, 50.0),
        CustomCollider::new(0.5),
    ));

    let wall_color1 = materials.add(Color::srgb(0.65, 0.32, 0.17));
    // let wall_color2 = materials.add(Color::srgb(0.55, 0.27, 0.07));

    // let origin = [25.0, 0.0, 25.0];

    let labyrinth = gen_map(cell_width, 10.0);

    for row in labyrinth {
        for b in &row {
            if b.w != 0.0 && b.h != 0.0 && b.d != 0.0 {
                commands.spawn((
                    RigidBody::Fixed,
                    Collider::cuboid(b.w/2.0, b.h/2.0, b.d/2.0),
                    Friction {
                        coefficient: 5.0,
                            combine_rule: CoefficientCombineRule::Max,
                    },
                    Restitution {
                        coefficient: 0.0,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    Transform::from_xyz(b.x, b.y, b.z),
                    Mesh3d(meshes.add(Cuboid::new(b.w, b.h, b.d))),
                    MeshMaterial3d(wall_color1.clone()),
                    CustomCollider::new(b.w.max(b.d)+2.0)
                ));
            }
        }
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
