// use crate::Vec3;
use bevy::prelude::*;
#[derive(Default, Component)]
pub struct Player {
    pub username: String,
    pub position: Vec3,
    pub dimension: Vec3,
    pub health: u32,
}

// trait

trait PlayerMethod {
    // fn move_to(&self, position: f32, direction: f32) -> (f32, f32);
    fn update_position(&mut self, position: Vec3);
}

impl Player {
    pub fn new(position: Vec3, dimension: Vec3, health: u32) -> Player {
        Player {
            username: "".to_string(),
            position,
            dimension,
            health,
        }
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn dimension(&self) -> Vec3 {
        self.dimension
    }
    pub fn health(&self) -> u32 {
        self.health
    }
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
    pub fn set_direction(&mut self, dimension: Vec3) {
        self.dimension = dimension;
    }
    pub fn set_health(&mut self, health: u32) {
        self.health = health;
    }
    // pub fn move_to(&self, position: f32, direction: f32) -> (f32, f32) {
    //     (position, direction)
    // }
}

impl PlayerMethod for Player {
    fn update_position(&mut self, position: Vec3) {
        self.position = position;
    }
}
