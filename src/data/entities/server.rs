use super::{Player, Game, UDP};
use std::net::TcpStream as Stream;
pub struct Server {
    clients: Vec<Player>,
    game: Game,
    network: UDP,
}


trait ServerMethod {
    fn accept(&self, stream: Stream) -> bool;
    fn broadcast(&self, message: String);
    fn manage_levels(&self);
}

impl Server {
    pub fn new(clients: Vec<Player>, game: Game, network: UDP) -> Server {
        Server {
            clients,
            game,
            network,
        }
    }
    pub fn clients(&self) -> &Vec<Player> {
        &self.clients
    }
    pub fn game(&self) -> &Game {
        &self.game
    }
    pub fn network(&self) -> UDP {
        self.network
    }
}