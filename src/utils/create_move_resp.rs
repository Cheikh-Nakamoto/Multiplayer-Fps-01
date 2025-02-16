use bevy::utils::HashMap;



pub fn create_move_resp(username : String,x:f32,y:f32,z:f32) -> HashMap<String,String> {
    let mut data = HashMap::new();
    data.insert("type".to_string(), "movement".to_string());
    data.insert("username".to_string(), username.clone());
    data.insert("x".to_string(), x.to_string());
    data.insert("y".to_string(), y.to_string());
    data.insert("z".to_string(), z.to_string());
    data
}