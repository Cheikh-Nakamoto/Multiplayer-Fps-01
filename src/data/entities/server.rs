use std::{io::Error, net::UdpSocket};

use super::{
    game::Game,
    player::Player,
    udp::{UDPMethod, UDP},
};
pub struct Server {
    clients: Vec<Player>,
    game: Game,
    pub network: UDP,
}

pub trait ServerMethod {
    fn accept(&self, stream: UdpSocket) -> Result<(), Error>;
    fn broadcast(&self, message: String);
    fn manage_levels(&self);
    async fn run(&self);
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
}

impl ServerMethod for Server {
    fn accept(&self, stream: UdpSocket) -> Result<(), Error> {
        Ok(())
    }
    fn broadcast(&self, message: String) {
        todo!()
    }
    fn manage_levels(&self) {}

    async fn run(&self) {
        loop {
            match self.network.receive().await {
                Ok((message,addr)) => {
                    println!("message recus: {} sur l'address {}", message,addr);
                }
                Err(e) => {
                    println!("error{}",e);
                }
            }
        }
    }
}
