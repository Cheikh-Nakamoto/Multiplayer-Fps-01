use std::io;
use std::io::{Error, ErrorKind};
use serde::{Deserialize, Serialize};
use super::{player::Player, udp::UDP};

pub struct Client {
    username: String,
    player: Player,
    pub network: UDP,
}

/*pub trait ClientMethods {
    fn update(&mut self);
    fn collect() -> Result<String,String>;
    fn connect(&self) -> Result<(String, String),Error>;
}
*/
impl Client {
    pub fn new(username: String, player: Player, network: UDP) -> Client {
        Client {
            username,
            player,
            network,
        }
    }
    pub fn ip_address(&self) -> String {
        self.network.address()
    }
    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn player(&self) -> &Player {
        &self.player
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
}

/*impl ClientMethods for Client {
    fn update(&mut self) {
        self.player.move_to(1.0, 1.0);
    }
    fn collect() -> Result<(String, String),Error> {
        let mut ip_addr = String::new();
        let mut username = String::new();
        println!("Veuillez entrer votre l'addresse du server :");
        io::stdin()
            .read_line(&mut ip_addr)
            .expect("Échec de la lecture de l'entrée");
        ip_addr = ip_addr.trim().to_string();
        println!("Veuillez entrer votre id de connexion:");
        io::stdin()
            .read_line(&mut username)
            .expect("Échec de la lecture de l'entrée");
        username = username.trim().to_string();
        Ok((username,ip_addr))
    }

    fn connect(&self) -> Result<(String, String), Error> {
        todo!()
    }
}
*/