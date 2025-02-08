use super::udp::UDPMethod;
use super::{player::Player, udp::UDP};
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::io::Error;


pub struct Client {
    username: String,
    player: Player,
    server: String,
    pub network: UDP,
}

pub trait ClientMethods {
    fn update(&mut self);
    fn collect() -> Result<(String, String), Error>;
    async fn connect(&mut self,username:String,ip_addr:String) -> Result<(String, String), Error>;
}

impl Client {
    pub fn new(username: String, player: Player, server: String, network: UDP) -> Client {
        Client {
            username,
            player,
            server,
            network,
        }
    }
    pub fn ip_address(&self) -> String {
        self.network.address()
    }
    pub fn username(&self) -> String {
        self.username.clone()
    }
    pub fn server(&self) -> String {
        self.server.clone()
    }

    pub fn player(&self) -> &Player {
        &self.player
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
}

impl ClientMethods for Client {
    fn update(&mut self) {
        self.player.move_to(1.0, 1.0);
    }
    fn collect() -> Result<(String, String), Error> {
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
        Ok((username, ip_addr))
    }

    async fn connect(&mut self,username:String,ip_addr:String) -> Result<(String, String), Error> {
        
        let mut identifient: HashMap<String, String> = HashMap::new();
        identifient.insert("username".to_string(), username.clone());
        let json_str = serde_json::to_string(&identifient).expect("Erreur de sérialisation");
        self.network
            .send(json_str, ip_addr.clone())
            .await
            .expect("TODO: panic message");
        let (message, ip) = self.network.receive().await.unwrap_or_default();
        let parsed_json: Value = serde_json::from_str(&message).expect("Erreur de parsing JSON");
        if let Some(req_status) =  parsed_json.get("status"){
            if req_status =="succes" {
                println!("Succes connexion !")
            }
        }
        println!("response source {} message {}", ip, message);
        Ok((username, ip_addr))
    }
}
