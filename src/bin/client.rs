use renet::{ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent};
use renet_steam::{AccessPermission, SteamClientTransport, SteamServerConfig, SteamServerTransport};
use steamworks::{Client, LobbyId, LobbyType, SteamId};

use std::{thread, time::Duration};

fn main() {
    // Setup steam client
    let steam_client = Client::init_app(480).unwrap();
    steam_client.networking_utils().init_relay_network_access();

    // Create renet client
    let connection_config = ConnectionConfig::default();
    let mut client = RenetClient::new(connection_config);

    // Create steam transport
    let server_steam_id = SteamId::from_raw(0); // Here goes the steam id of the host
    let mut steam_transport = SteamClientTransport::new(steam_client.clone(), &server_steam_id).unwrap();

    // Your gameplay loop
    loop {
        let delta_time = Duration::from_millis(16);

        steam_client.run_callbacks(); // Update steam callbacks
        client.update(delta_time);
        steam_transport.update(&mut client);

        // Code for sending/receiving messages can go here
        // Check the examples/demos 

        steam_transport.send_packets(&mut client).unwrap();
        thread::sleep(delta_time);
    }
}