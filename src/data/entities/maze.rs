use super::Wall;
pub struct Maze {
    walls: Vec<Wall>,
    spawn_points: Vec3,
}

trait MazeMethod {
    fn generate(&self);
    fn is_wall(&self) -> bool;
}

impl Maze {
    pub fn new() -> Maze {
        Maze {
            walls: Vec::new(),
            spawn_points: Vec3::new(),
        }
    }
    pub fn walls(&self) -> &Vec<Wall> {
        &self.walls
    }
    pub fn spawn_points(&self) -> Vec3 {
        self.spawn_points
    }
}