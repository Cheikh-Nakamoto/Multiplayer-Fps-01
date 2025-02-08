use std::{io::Error, net::UdpSocket};
use serde::{Deserialize};
use std::collections::HashMap;
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
                Ok((message, addr)) => {
                    dbg!("Okk");
                    println!("message recus: {} sur l'address {}", message,addr);
                    match serde_json::from_str::<HashMap<String, String>>(&message) {
                        Ok(information) => {
                            let mut msg : HashMap<String, String> = HashMap::new();
                            msg.insert("status".to_string(), "succes".to_string());
                            let json_msg = serde_json::to_string(&msg).expect("Error");
                            if let Some(_) = information.get("username") {
                               self.network.send(json_msg, addr).await.expect("Error");
                            }
                        },
                        Err(e) => {
                            dbg!("Error", e);
                        },
                    }
                    ;
                }
                Err(e) => {
                    println!("error{}",e);
                }
            }
        }
    }
}
