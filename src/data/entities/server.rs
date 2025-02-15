use crate::data::enums::type_msg::TypeMessage;

use super::{
    clients::Client,
    game::Game,
    player::Player,
    udp::{UDPMethod, UDP},
};
use serde::Deserialize;
use std::collections::HashMap;
use std::{io::Error, net::UdpSocket};

pub struct Server {
    pub addr_clients: Vec<String>,
    pub players: Vec<Player>,
    pub game: Game,
    pub network: UDP,
}

pub trait ServerMethod {
    fn accept(&mut self, username: String, addr: String);
    async fn broadcast(&self, data: HashMap<String, String>);
    fn manage_levels(&self);
    async fn run(server: &mut Server);
    async fn response(&self, data: HashMap<String, String>, ip_adrrs: String, status: &str);
    async fn treatment_message(
        &mut self,
        adrr_client: String,
        data: TypeMessage,
        information: HashMap<String, String>,
    );
}
impl Server {
    pub fn new(clients: Vec<String>, players: Vec<Player>, game: Game, network: UDP) -> Server {
        Server {
            addr_clients: clients,
            players,
            game,
            network,
        }
    }

    pub async fn check_username(&self, data: HashMap<String, String>) -> Result<(), Error> {
        println!("len players {}", self.players.len());
        for player in self.players.iter() {
            println!("Checking player {}", player.username);
        }
        if let Some(username) = data.get("username") {
            if self.get_player_by_username(username).is_some() || username == "" {
                self.response(data, self.network.address(), "Nom d'utilisateur vide").await;
                return Err(Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "Nom d'utilisateur vide",
                ));
            }
        } else {
            return Err(Error::new(
                std::io::ErrorKind::NotConnected,
                "Veuillez verifier le type",
            ));
        }
        self.response(data, self.network.address(), "succes").await;
        Ok(())
    }
    pub fn game(&self) -> &Game {
        &self.game
    }

    fn get_player_by_username(&self, username: &str) -> Option<&Player> {
        for player in self.players.iter() {
            if player.username == username {
                return Some(player);
            }
        }
        None
    }
}

impl ServerMethod for Server {
    fn accept(&mut self, username: String, addr: String) {
        let mut player = Player::default();
        player.username = username.to_string();
        self.addr_clients.push(addr.clone());
        self.players.push(player);
        println!("Nouveau client : {}", username);
    }
    async fn broadcast(&self, data: HashMap<String, String>) {
        for addr in &self.addr_clients {
            self.response(data.clone(), addr.clone(), "succes").await;
        }
    }

    async fn response(&self, data: HashMap<String, String>, ip_adrrs: String, status: &str) {
        let mut msg: HashMap<String, String> = data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone())) // Clone les clés et les valeurs
            .collect();
        msg.insert("status".to_string(), status.to_string());
        let json_msg = serde_json::to_string(&msg).expect("Error");
        self.network
            .send(json_msg, format!("{}:8081", ip_adrrs))
            .await
            .expect("Error");
    }

    fn manage_levels(&self) {}

    async fn treatment_message(
        &mut self,
        addr: String,
        message: TypeMessage,
        information: HashMap<String, String>,
    ) {
        match message {
            TypeMessage::Connection => {
                let username = information.get("username").unwrap().to_string();
                match self.check_username(information).await {
                    Ok(()) => {
                        self.accept(username, addr.clone());
                    }
                    Err(_) => {
                        dbg!("Error {}", username);
                    }
                }
            }
            TypeMessage::Movement => {
                println!("Movement");
            }
            TypeMessage::Disconnection => {
                println!("Disconnect");
            }
            TypeMessage::Unknown => {
                println!("Unknown");
            }
        }
    }

    async fn run(server: &mut Server) {
        loop {
            match server.network.receive().await {
                Ok((message, addr)) => {
                    println!(
                        "message decode : {:?}",
                        serde_json::from_str::<HashMap<String, String>>(&message)
                    );
                    match serde_json::from_str::<HashMap<String, String>>(&message) {
                        Ok(information) => {
                            let type_msg =
                                TypeMessage::from(information.get("type").unwrap().as_str());
                            server
                                .treatment_message(addr.clone(), type_msg, information.clone())
                                .await;
                        }
                        Err(e) => {
                            println!("Error : {}", e);
                            server
                                .response(HashMap::new(), addr.clone(), "failed")
                                .await;
                        }
                    }
                }
                Err(e) => {
                    println!("error{}", e);
                }
            }
        }
    }
}
