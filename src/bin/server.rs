use renet::{ClientId, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};

use std::{net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};

use lethallib::server::ServerState;

fn main() {
    

    // // Your gameplay loop
    // loop {
    //     let delta_time = Duration::from_millis(16);
    //     // Receive new messages and update clients
    //     server.update(delta_time);
    //     let _ = transport.update(delta_time, &mut server);
        
    //     // Check for client connections/disconnections
    //     while let Some(event) = server.get_event() {
    //         match event {
    //             ServerEvent::ClientConnected { client_id } => {
    //                 println!("Client {client_id} connected");
    //             }
    //             ServerEvent::ClientDisconnected { client_id, reason } => {
    //                 println!("Client {client_id} disconnected: {reason}");
    //             }
    //         }
    //     }

    //     // Receive message from channel
    //     for client_id in server.clients_id() {
    //         // The enum DefaultChannel describe the channels used by the default configuration
    //         while let Some(_message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
    //             // Handle received message
    //         }
    //     }
        
    //     // Send a text message for all clients
    //     server.broadcast_message(DefaultChannel::ReliableOrdered, "server message");

    //     let client_id: ClientId = 0;
    //     // Send a text message for all clients except for Client 0
    //     server.broadcast_message_except(client_id, DefaultChannel::ReliableOrdered, "server message");
        
    //     // Send message to only one client
    //     server.send_message(client_id, DefaultChannel::ReliableOrdered, "server message");
    
    //     // Send packets to clients using the transport layer
    //     transport.send_packets(&mut server);

    //     std::thread::sleep(delta_time); // Running at 60hz
    // }

    let target_fps = 60;
    let target_dt = 1.0 / target_fps as f64;
    let mut dt_err = 0.0;

    let mut prev_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time should go forward");

    let mut state: ServerState = ServerState::Startup;

    let mut serveroption: Option<RenetServer> = None;
    let mut transportoption: Option<NetcodeServerTransport> = None;

    loop {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time should go forward");
        let dt = (time - prev_time).as_secs_f64();

        match state {
            ServerState::Startup => {
                serveroption = None;
                transportoption = None;

                let server = RenetServer::new(ConnectionConfig::default());

                // Setup transport layer using renet_netcode
                const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000);
                let socket: UdpSocket = UdpSocket::bind(SERVER_ADDR).unwrap();
                let server_config = ServerConfig {
                    current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
                    max_clients: 64,
                    protocol_id: 0,
                    public_addresses: vec![SERVER_ADDR],
                    authentication: ServerAuthentication::Unsecure
                };
                let mut transport = NetcodeServerTransport::new(server_config, socket).unwrap();

                serveroption = Some(server);
                transportoption = Some(transport);

                state = ServerState::Lobby;
            },
            ServerState::Lobby => {
                let server = serveroption.as_mut().unwrap();
                let transport = transportoption.as_mut().unwrap();
            },
            ServerState::InGame => {

            },
            ServerState::Close => {
                break;
            },
        }

        dt_err = (0.9 * dt_err) + (0.1 * (target_dt - dt));
        std::thread::sleep(Duration::from_secs_f64(dt_err.max(0.0)));

        println!("Framerate: {}", 1.0 / dt);

        prev_time = time;
    }
}