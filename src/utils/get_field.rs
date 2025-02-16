

use bevy::{math::Vec3, utils::HashMap};

pub fn get_pos_player(data: HashMap<String, String>) -> Vec3 {
    let x = data
        .get("x")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(0.0);
    let y = data
        .get("y")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(0.0);
    let z = data
        .get("z")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(0.0);
    Vec3::new(x, y, z)
}

pub fn get_field(data: HashMap<String, String>,field_name:&str) -> String {
   if let Some(field) = data.get(field_name) {
        return  field.clone();
   }
   return String::new()
}
