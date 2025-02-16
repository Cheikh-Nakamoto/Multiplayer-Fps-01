// use bevy::render::render_resource::Texture;
use crate::Vec3;

pub struct Wall {
    position: Vec3,
    width: f32,
    height: f32,
    //texture: Texture,
}


pub trait WallMethod {
    fn collides(&self, position: Vec3) -> Result<(), String>;
    fn render(&self);
}

impl Wall {
    pub fn new(position: Vec3, width: f32, height: f32) -> Wall {
        Wall {
            position,
            width,
            height,
          //  texture,
        }
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
}

impl  WallMethod for Wall {

    fn collides(&self, position: Vec3) -> Result<(), String> {
        if position.x >= self.position.x 
            && position.x <= self.position.x + self.width
            && position.y >= self.position.y 
            && position.y <= self.position.y + self.width
            && position.z >= self.position.z 
            && position.z <= self.position.z + self.width {
            Ok(()) 
        } else {
            Err("Colision".to_string()) 
        }
    }

    fn render(&self) {

    }
    
}