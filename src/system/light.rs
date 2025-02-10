use bevy::app::Plugin;
use bevy::color::Srgba;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::{default, Bundle, Color, Commands, DirectionalLight, Startup, Transform, Vec3};
#[derive(Default)]
pub struct LigthPlugin ;

impl Plugin for LigthPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_light);
    }
}

fn spawn_light(mut command: Commands) {
    // the sun:
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
        .build();

    // Sun
    command.spawn((
        DirectionalLight {
            color: Color::Srgba(Srgba::rgb(0.98, 0.95, 0.82)),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        cascade_shadow_config,
    ));
}
