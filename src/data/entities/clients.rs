use super::udp::UDPMethod;
use super::{player::Player, udp::UDP};
use bevy::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::io::Error;

#[derive(Clone,Resource)]
pub struct Client {
    username: String,
    player: Player,
    server: String,
}

pub trait ClientMethods {
    // fn update(&mut self);
    fn collect() -> Result<(String, String), Error>;
    fn connect(
        &mut self,
        username: String,
        ip_addr: String,
    ) -> impl std::future::Future<Output = Result<(String, String), Error>> + Send;
}

impl Client {
    pub fn new(username: String, player: Player, server: String) -> Client {
        Client {
            username,
            player,
            server,
        }
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

    async fn connect(
        &mut self,
        username: String,
        ip_addr: String,
    ) -> Result<(String, String), Error> {
        let socket = UDP::create_socket_sender(8081).await?;
        let mut identifient: HashMap<String, String> = HashMap::new();
        identifient.insert("type".to_string(), "connection".to_string());
        identifient.insert("username".to_string(), username.clone());
        let json_str = serde_json::to_string(&identifient).expect("Erreur de sérialisation");
        socket.send(json_str, ip_addr.clone()).await?;
        let (message, ip) = socket.receive().await.unwrap_or_default();
        let parsed_json: Value = serde_json::from_str(&message).expect("Erreur de parsing JSON");
        if let Some(req_status) = parsed_json.get("status") {
            if req_status != "succes" {
                return Err(Error::new(
                    std::io::ErrorKind::NotConnected,
                    format!("{}",req_status ),
                ));
            }
        }
        println!("response source {} message {}", ip, message);
        Ok((username, ip_addr))
    }
}

