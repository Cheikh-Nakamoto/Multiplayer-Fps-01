use bevy::{
    prelude::*,
    render::{
        camera::{ScalingMode, Viewport},
        view::RenderLayers,
    },
};

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_minimap_view);
    }
}

// Composant pour identifier la caméra de la minimap
#[derive(Component)]
struct MinimapCamera;

fn setup_minimap_view(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();

    // Configuration de la taille de la minimap
    // 300x200 pixels pour avoir une vue d'ensemble du labyrinthe généré
    let minimap_size = Vec2::new(300.0, 200.0);
    let safe_width = window.width().max(300.0);
    
    // Positionnement de la minimap en haut à droite
    // La position est calculée en fonction de la taille de la fenêtre
    let minimap_position = Vec2::new(safe_width - minimap_size.x - 20.0, 0.0);

    // Protection contre les fenêtres invalides
    if window.width() == 0.0 || window.height() == 0.0 {
        return;
    }

    // Dimensions de la vue de la map
    // Ces valeurs doivent correspondre à l'échelle du labyrinthe généré
    let map_width = 200.0;  // Doit couvrir la largeur totale du labyrinthe
    let map_height = 200.0; // Doit couvrir la hauteur totale du labyrinthe
    
    // Facteur de zoom pour ajuster la vue
    // 0.5 signifie que la caméra voit une zone deux fois plus grande
    let zoom_factor = 0.5;
    let view_width = map_width * zoom_factor;
    let view_height = map_height * zoom_factor;

    // Création de la caméra de la minimap
    commands.spawn((
        Camera3d::default(),
        Camera {
            viewport: Some(Viewport {
                physical_position: minimap_position.as_uvec2(), // Position à l'écran
                physical_size: minimap_size.as_uvec2(),        // Taille à l'écran
                ..default()
            }),
            // Ordre 999 pour s'assurer que la minimap est dessinée par-dessus tout le reste
            order: 999,
            ..default()
        },
        // Position et orientation de la caméra
        // Hauteur de 200 unités pour avoir une vue d'ensemble du labyrinthe
        Transform::from_xyz(0.0, 200.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        // Configuration de la projection orthographique
        // Utilise une vue fixe pour maintenir les proportions
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: view_width,
                height: view_height,
            },
            ..OrthographicProjection::default_3d()
        }),
        MinimapCamera,
        // La caméra peut voir les layers 0 et 1
        // Layer 0 : éléments du jeu principal (murs, sol)
        // Layer 1 : éléments spécifiques à la minimap
        RenderLayers::from_layers(&[0,1]),
    ));
}