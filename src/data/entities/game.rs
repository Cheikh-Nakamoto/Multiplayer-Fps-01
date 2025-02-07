use super::{Player, Maze};
pub struct Game {
    players: Vec<Player>,
    maze: Maze,
    level: u32,
}


trait GameMethod {
    fn update(&self);
    fn check_collision(&self);
    fn spawn_player(&self);
}


impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
            maze: Maze::new(),
            level: 0,
        }
    }
    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }
    pub fn maze(&self) -> &Maze {
        &self.maze
    }
    pub fn level(&self) -> u32 {
        self.level
    }
}