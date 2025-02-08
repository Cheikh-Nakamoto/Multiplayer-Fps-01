
use std::io::{self, Error};
use tokio::net::UdpSocket;
pub struct UDP {
    pub socket: UdpSocket,
}

pub trait UDPMethod {
    async fn send(&self, message: String, addr: String) -> Result<usize, Error> ;
    async fn connect_to_dest(&self, ip_addr: String) -> Result<(), Error>;
    async fn receive(&self) ->Result<(String, String), Error> ;
}

impl UDP {
    pub async fn new(port: u32, address: &str) -> Result<UDP, Error> {
        let socket = UdpSocket::bind(format!("{}:{}",address,port)).await?;
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
            addr
        };
        println!("{}",addr_with_port);
        if addr_with_port.parse::<std::net::SocketAddr>().is_err() {
            eprintln!("❌ Erreur : Adresse invalide '{}'", addr_with_port);
            return Err(Error::new(std::io::ErrorKind::InvalidInput, "Adresse invalide"));
        }
        Ok(self.socket.send_to(message.as_bytes(), addr_with_port).await?)
    }


    async fn connect_to_dest(&self, ip_addr: String) -> Result<(), Error> {
        Ok(self.socket.connect(ip_addr).await?)
    }

    async fn receive(&self) -> Result<(String, String), Error> {
        let mut message = Vec::new();
        let mut buf = [0; 8192];
        let mut source = String::new();

        match self.socket.recv_from(&mut buf).await? {
            (n, addr) => {
                message.extend_from_slice(&buf[..n]);
                source = addr.ip().to_string();
            }
            // Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            //     continue;
            // }
            e => return Err(Error::new(io::ErrorKind::BrokenPipe,e.1.ip().to_string())),
        }
        Ok((String::from_utf8_lossy(&message).to_string(),source))
    }
}
