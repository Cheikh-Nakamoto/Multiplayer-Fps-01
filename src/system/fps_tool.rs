use bevy::{prelude::*, render::view::RenderLayers, text::FontSmoothing};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

pub struct OverlayColorPlugin;

impl OverlayColorPlugin {
    //const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

impl Plugin for OverlayColorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    // Here we define size of our overlay
                    font_size: 22.0,
                    // If we want, we can use a custom font
                    font: default(),
                    // We could also disable font smoothing,
                    font_smoothing: FontSmoothing::default(),
                },
                // We can also change color of the overlay
                text_color: OverlayColorPlugin::GREEN,
                enabled: true,
            },
        });
        //app.add_systems(Startup, setup);
        //app.add_systems(Update, customize_config);
    }
}



// fn customize_config(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
//     if input.just_pressed(KeyCode::Digit1) {
//         // Changing resource will affect overlay
//         if overlay.text_color == OverlayColorPlugin::GREEN {
//             overlay.text_color = OverlayColorPlugin::RED;
//         } else {
//             overlay.text_color = OverlayColorPlugin::GREEN;
//         }
//     }
//     if input.just_pressed(KeyCode::Digit2) {
//         overlay.text_config.font_size -= 2.0;
//     }
//     if input.just_pressed(KeyCode::Digit3) {
//         overlay.text_config.font_size += 2.0;
//     }
//     if input.just_pressed(KeyCode::Digit4) {
//         overlay.enabled = !overlay.enabled;
//     }
// }
