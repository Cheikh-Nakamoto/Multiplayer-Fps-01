use bevy::prelude::*;
use std::io::Error;
use tokio::runtime::Runtime;
use multiplayer_fps::data::entities::clients::{ Client, ClientMethods};
use multiplayer_fps::data::entities::player::Player;
use multiplayer_fps::system::functions_system::{move_client_system, setup_system};

// #[tokio::main]
fn main() -> Result<(), Error> {
    let rt = Runtime::new().unwrap();
    
    let client = rt.block_on(async {
        let (username, ip_addr) = Client::collect()?;
        let mut client = Client::new(username.clone(), Player::default(), ip_addr.clone());
        client.connect(username, ip_addr).await?;
        Ok::<_, Error>(client)
    })?;

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(client)
        .add_systems(Startup, setup_system)
        .add_systems(Update, move_client_system)
        .run();

    Ok(())
}
