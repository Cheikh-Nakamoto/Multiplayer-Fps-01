use bevy::utils::HashMap;

use crate::{
    data::enums::type_msg::TypeMessage,
    utils::{
        create_move_resp::create_move_resp,
        get_field::{get_field, get_pos_player},
    },
};

use super::{
    game::Game,
    player::Player,
    udp::{UDPMethod, UDP},
};
use std::io::Error;

pub struct Server {
    pub addr_clients: Vec<String>,
    pub players: Vec<Player>,
    pub game: Game,
    pub network: UDP,
}

pub trait ServerMethod {
    fn accept(
        &mut self,
        username: String,
        addr: String,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn broadcast(
        &self,
        data: HashMap<String, String>,
        addr_client: String,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn manage_levels(&self);
    fn run(server: &mut Server) -> impl std::future::Future<Output = ()> + Send;
    fn response(
        &self,
        data: HashMap<String, String>,
        ip_adrrs: String,
        status: &str,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn treatment_message(
        &mut self,
        adrr_client: String,
        data: TypeMessage,
        information: HashMap<String, String>,
    ) -> impl std::future::Future<Output = ()> + Send;
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

    pub async fn check_username(
        &self,
        data: HashMap<String, String>,
        addr: String,
    ) -> Result<(), Error> {
        println!("len players {}", self.players.len());
        for player in self.players.iter() {
            println!("Checking player {}", player.username);
        }
        if let Some(username) = data.get("username") {
            if self.get_player_by_username(username).is_some() || username == "" {
                self.response(data, addr, "Nom d'utilisateur incorrect ou déjà utilisé")
                    .await;
                return Err(Error::new(
                    std::io::ErrorKind::NotConnected,
                    "Nom d'utilisateur incorrect ou déjà utilisé",
                ));
            }
        } else {
            return Err(Error::new(
                std::io::ErrorKind::NotConnected,
                "Veuillez verifier le type",
            ));
        }
        self.response(data, addr, "succes").await;
        Ok(())
    }

    fn get_player_by_username(&self, username: &str) -> Option<&Player> {
        for player in self.players.iter() {
            if player.username == username {
                return Some(player);
            }
        }
        None
    }
    fn group_players_by_username(&self, curentjoin: String) -> HashMap<String, String> {
        let mut player_map: HashMap<String, String> = HashMap::new();
        for player in &self.players {
            if curentjoin != player.username {
                let str = serde_json::to_string(&player.position());
                player_map.insert(player.username.clone(), str.unwrap_or_default());
            }
        }

        player_map.insert("type".to_owned(), "participants".to_string());

        player_map
    }
}

impl ServerMethod for Server {
    async fn accept(&mut self, username: String, addr: String) {
        let mut player = Player::default();
        player.username = username.clone();
        self.addr_clients.push(addr.clone());
        self.players.push(player);

        println!("Nouveau client : {} de {}", username, addr);
        println!("adress enregistre a la connexion: {:?}", &self.addr_clients);

        // Créer les données à diffuser
        let mut new_player_request = HashMap::new();
        new_player_request.insert("type".to_string(), "join".to_string()); // Type de message
        new_player_request.insert("username".to_string(), username.clone()); // Nom du joueur
        new_player_request.insert("addr".to_string(), addr.clone()); // Adresse du joueur

        let _type_msg = TypeMessage::from("join");
        // self.response(participants, addr.clone(), "succes").await;
        // Diffuser les données à tous les clients
        self.broadcast(new_player_request, addr.clone())
            .await;
    }
    async fn broadcast(
        &self,
        player: HashMap<String, String>,
        addr_client: String,
    ) {
        println!("broadcast to...");
        for addr in &self.addr_clients {
            if addr.to_owned().trim() != addr_client.trim() {
                println!("====>... {}", addr);
                self.response(player.clone(), addr.clone(), "succes").await;
            }
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
        let username = get_field(information.clone(), "username");
        match message {
            TypeMessage::Connection => match self.check_username(information, addr.clone()).await {
                Ok(()) => {
                    self.accept(username, addr.clone()).await;
                }
                Err(e) => {
                    dbg!("Error {}", e);
                }
            },
            TypeMessage::Movement => {
                println!("adress des client: {:?}", &self.addr_clients);

                let pos = get_pos_player(information);
                println!("<======================================>");
                println!("<===========Update user: {}============>", username);
                for player in self.players.iter_mut() {
                    if player.username == username {
                        println!(
                            "<===============player find in pos {:?}================>",
                            pos
                        );
                        player.position = pos;
                        break;
                    }
                }
                println!("<===============finish================>");
                let data = create_move_resp(username, pos.x, pos.y, pos.z);
                self.broadcast(data, addr.clone())
                    .await;
            }
            TypeMessage::Disconnection => {
                println!("Disconnect");
            }
            TypeMessage::Unknown => {
                println!("Unknown");
            }
            TypeMessage::Participants => {
                let participants = self.group_players_by_username(username);
                self.response(participants, addr, "succes")
                    .await;
            }
            _ => todo!(),
        }
    }

    async fn run(server: &mut Server) {
        loop {
            match server.network.receive().await {
                Ok((message, addr)) => {
                    println!(
                        "message decode : {:?} provenant de {}",
                        serde_json::from_str::<HashMap<String, String>>(&message),
                        addr
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
