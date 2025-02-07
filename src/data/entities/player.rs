pub struct Player {
    position: (f32, f32),
    direction: f32,
    health: u32
}

// trait

trait PlayerMethod {
   fn move_to(&self, position: f32, direction: f32) -> (f32, f32);
}

impl Player {
    pub fn new(position: (f32, f32), direction: f32, health: u32) -> Player {
        Player {
            position,
            direction,
            health
        }
    }
    pub fn position(&self) -> (f32, f32) {
        self.position
    }
    pub fn direction(&self) -> f32 {
        self.direction
    }
    pub fn health(&self) -> u32 {
        self.health
    }
    pub fn set_position(&mut self, position: (f32, f32)) {
        self.position = position;
    }
    pub fn set_direction(&mut self, direction: f32) {
        self.direction = direction;
    }
    pub fn set_health(&mut self, health: u32) {
        self.health = health;
    }
    pub fn move_to(&self, position: f32, direction: f32) -> (f32, f32) {
        (position, direction)
    }
}