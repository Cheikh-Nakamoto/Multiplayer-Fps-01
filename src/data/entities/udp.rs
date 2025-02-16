use bevy::{ecs::system::Resource, utils::HashMap};
use get_if_addrs::get_if_addrs;
use std::io::Error;
use tokio::{net::UdpSocket, sync::mpsc};
pub struct UDP {
    pub socket: UdpSocket,
}
#[derive(Resource)]
pub struct UdpReceiver {
    pub receiver: mpsc::Receiver<HashMap<String, String>>,
}
pub trait UDPMethod {
    fn send(&self, message: String, addr: String) -> impl std::future::Future<Output = Result<usize, Error>> + Send;
    fn receive(&self) -> impl std::future::Future<Output = Result<(String, String), Error>> + Send;
    fn create_socket_sender(port: u32) -> impl std::future::Future<Output = Result<UDP, Error>> + Send;
}

impl UDP {
    pub async fn new(port: u32, address: &str) -> Result<UDP, Error> {
        let socket = UdpSocket::bind(format!("{}:{}", address, port)).await?;
        socket.set_broadcast(true)?; // Permettre la réception en broadcast
        Ok(UDP { socket })
    }

    pub fn port(&self) -> u32 {
        self.socket.local_addr().unwrap().port() as u32
    }
    pub fn address(&self) -> String {
        self.socket.local_addr().unwrap().ip().to_string()
    }
}

impl UDPMethod for UDP {
    async fn send(&self, message: String, addr: String) -> Result<usize, Error> {
        println!("Message envoyé : {} vers {}", message, addr);
        let addr_with_port = if !addr.contains(":") {
            format!("{}:8080", addr.trim()) // Ajoute le port 8080 par défaut
        } else {
            addr.to_string()
        };
        if addr_with_port.parse::<std::net::SocketAddr>().is_err() {
            println!("❌ Erreur : Adresse invalide '{}'", addr_with_port);
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Adresse invalide",
            ));
        }

        match self
            .socket
            .send_to(message.as_bytes(), addr_with_port.clone())
            .await
        {
            Ok(n) => println!(" message len: {}", n),
            Err(e) => {
                println!(
                    "Error {:?} validity address {:?}",
                    e,
                    addr_with_port.parse::<std::net::SocketAddr>().is_err()
                );
            }
        }
        Ok(0)
    }

    async fn create_socket_sender(port: u32) -> Result<UDP, Error> {
        let interfaces = get_if_addrs().expect("Impossible de récupérer les interfaces réseau");
        let ip_client = interfaces[1].ip().to_string();
        println!("IP client : {}:{}", ip_client, port);
        let udp = UDP::new(port, ip_client.as_str()).await?;
        Ok(udp)
    }

    async fn receive(&self) -> Result<(String, String), Error> {
        let mut message = Vec::new();
        let mut buf = [0; 8192];
        let source ;
        println!("Received");
        match self.socket.recv_from(&mut buf).await? {
            (n, addr) => {
                message.extend_from_slice(&buf[..n]);
                source = addr.ip().to_string();
            }
            // e => return Err(Error::new(io::ErrorKind::BrokenPipe, e.1.ip().to_string())),
        }
        Ok((String::from_utf8_lossy(&message).to_string(), source))
    }
}
