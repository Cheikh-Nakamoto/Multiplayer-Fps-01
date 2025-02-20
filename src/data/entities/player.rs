// use crate::Vec3;
use bevy::prelude::*;
#[derive(Default, Debug, Clone, Component)]
pub struct Player {
    pub username: String,
    pub position: Vec3,
    pub dimension: Vec3,
    pub health: u32,
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

#[derive(Debug, Clone)]
pub struct PlayerGroup {
    pub player: Vec<Player>,
    pub message: String,
}

impl Player {
    pub fn new() -> Player {
        Player {
            username: "".to_string(),
            position: Vec3::ZERO,
            dimension: Vec3::ZERO,
            health: 1000,
            movement_speed: 10.0,
            rotation_speed: f32::to_radians(360.0),
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
