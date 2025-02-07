use std::texture::Texture;

pub struct Wall {
    position: (f32, f32),
    width: f32,
    height: f32,
    texture: Texture,
}


trait WallMethod {
    fn collides(&self, other: &Wall) -> bool;
    fn render(&self);
}

impl Wall {
    pub fn new(position: (f32, f32), width: f32, height: f32, texture: Texture) -> Wall {
        Wall {
            position,
            width,
            height,
            texture,
        }
    }
    pub fn position(&self) -> (f32, f32) {
        self.position
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn texture(&self) -> Texture {
        self.texture
    }
}