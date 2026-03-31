use renet::{ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent};
use renet_steam::{AccessPermission, SteamClientTransport, SteamServerConfig, SteamServerTransport};
use steamworks::{Client, LobbyId, LobbyType, SteamId};

use std::{thread, time::Duration};

fn main() {
    // Setup steam client
    let steam_client = Client::init_app(480).unwrap();
    steam_client.networking_utils().init_relay_network_access();

    // Create renet server
    let connection_config = ConnectionConfig::default();
    let mut server: RenetServer = RenetServer::new(connection_config);

    // Create steam transport
    let access_permission = AccessPermission::Public;
    let steam_transport_config = SteamServerConfig {
        max_clients: 32,
        access_permission,
    };
    let mut steam_transport = SteamServerTransport::new(steam_client.clone(), steam_transport_config).unwrap();

    // Your gameplay loop
    loop {
        let delta_time = Duration::from_millis(16);

        steam_client.run_callbacks(); // Update steam callbacks
    
        server.update(delta_time);
        steam_transport.update(&mut server);

        // Handle connect/disconnect events
        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("Client {} connected.", client_id)
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {} disconnected: {}", client_id, reason);
                }
            }
        }

        // Code for sending/receiving messages can go here
        // Check the examples/demos 

        steam_transport.send_packets(&mut server);
        thread::sleep(delta_time);
    }
}