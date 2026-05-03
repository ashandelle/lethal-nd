use renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bincode::error::DecodeError;

use std::{net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::{Duration, SystemTime, UNIX_EPOCH}};

use lethallib::{client::{ReliableClientMessage, UnreliableClientMessage}, disconnectreason::DisconnectReason, server::{ReliableServerMessage, ServerMessageVisibility, ServerState, UnreliableServerMessage}, timer::Timer, world::world::World};

fn main() {
    const N: usize = 3;

    let target_fps = 60;
    let target_dt = 1.0 / target_fps as f64;
    let mut dt_err = 0.0;

    let mut prev_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time should go forward");

    let mut state: ServerState = ServerState::Startup;

    let mut serveroption: Option<RenetServer> = None;
    let mut transportoption: Option<NetcodeServerTransport> = None;
    let mut worldoption: Option<World<N>> = None;

    let mut debugtimer: Timer = Timer::new(10.0);

    loop {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time should go forward");
        let dt = (time - prev_time).as_secs_f64();

        debugtimer.advance(dt);

        match state {
            ServerState::Startup => {
                // serveroption = None;
                // transportoption = None;
                // worldoption = None;

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
                let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

                serveroption = Some(server);
                transportoption = Some(transport);
                worldoption = Some(World::new_server());

                state = ServerState::Connected;//{ connectedstate: ServerConnectedState::Lobby };
                statechanged(&state, &mut debugtimer);
            },
            ServerState::Connected => {
                macro_rules! receive_messages {
                    ($server:ident, $messages_name:ident, $message_type:ty, $channel_id:expr) => {
                        let mut $messages_name: Vec<(u64, $message_type)> = Vec::new();

                        for client_id in $server.clients_id() {
                            // The enum DefaultChannel describe the channels used by the default configuration
                            while let Some(message) = $server.receive_message(client_id, $channel_id) {
                                let message: Result<($message_type, usize), DecodeError>  = bincode::decode_from_slice(&message, bincode::config::standard());

                                match message {
                                    Ok((servermessage, _)) => {
                                        $messages_name.push((client_id, servermessage));
                                    },
                                    Err(err) => {
                                        println!("Error: {:?}", err);
                                    }
                                }
                            }
                        }
                    };
                }

                macro_rules! send_messages {
                    ($server:ident, $messages_name:ident, $message_type:ty, $channel_id:expr) => {
                        for (visibility, message) in $messages_name.iter() {
                            let message = bincode::encode_to_vec(&message, bincode::config::standard()).unwrap();
                            match *visibility {
                                lethallib::server::ServerMessageVisibility::Broadcast => {
                                    $server.broadcast_message($channel_id, message);
                                },
                                lethallib::server::ServerMessageVisibility::Except { id } => {
                                    $server.broadcast_message_except(id, $channel_id, message);
                                },
                                lethallib::server::ServerMessageVisibility::Only { id } => {
                                    $server.send_message(id, $channel_id, message);
                                },
                            }
                        }
                    };
                }

                let server = serveroption.as_mut().unwrap();
                let transport = transportoption.as_mut().unwrap();
                let world = worldoption.as_mut().unwrap();

                let delta_time = Duration::from_secs_f64(dt);
                // Receive new messages and update clients
                server.update(delta_time);
                match transport.update(delta_time, server) {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Error: {:?}", err);
                    },
                }

                let reliablemessagessent: Vec<(ServerMessageVisibility, ReliableServerMessage<N>)> = Vec::new();
                let unreliablemessagessent: Vec<(ServerMessageVisibility, UnreliableServerMessage<N>)> = Vec::new();

                world.server_set_channels(reliablemessagessent, unreliablemessagessent);
        
                // Check for client connections/disconnections
                while let Some(event) = server.get_event() {
                    match event {
                        ServerEvent::ClientConnected { client_id } => {
                            world.player_connected(client_id);
                            println!("Client {client_id} connected");
                        }
                        ServerEvent::ClientDisconnected { client_id, reason } => {
                            let publicreason = match reason {
                                renet::DisconnectReason::DisconnectedByClient => DisconnectReason::Left,
                                renet::DisconnectReason::DisconnectedByServer => DisconnectReason::Kicked,
                                _ => DisconnectReason::NetworkError,
                            };
                            world.player_disconnected(client_id, publicreason);
                            println!("Client {client_id} disconnected: {reason}");
                        }
                    }
                }

                receive_messages!(server, reliablemessagesreceived, ReliableClientMessage<N>, DefaultChannel::ReliableOrdered);
                receive_messages!(server, unreliablemessagesreceived, UnreliableClientMessage<N>, DefaultChannel::Unreliable);

                world.process_reliable_client_messages(reliablemessagesreceived);
                world.process_unreliable_client_messages(unreliablemessagesreceived);
                world.server_update(dt);

                let (reliablemessagessent, unreliablemessagessent) = world.server_extract_channels();
                send_messages!(server, reliablemessagessent, ReliableServerMessage<N>, DefaultChannel::ReliableOrdered);
                send_messages!(server, unreliablemessagessent, UnreliableServerMessage<N>, DefaultChannel::Unreliable);
    
                // Send packets to clients using the transport layer
                transport.send_packets(server);
            },
            ServerState::Close => {
                break;
            },
        }

        if debugtimer.is_elapsed() {
            debugtimer.partial_reset();

            println!("Serverstate: {:?}", state);

            if let Some(world) = &worldoption {
                println!("World: {:?}", world);
            }
        }

        dt_err += target_dt - dt;
        dt_err = dt_err.max(0.0);
        std::thread::sleep(Duration::from_secs_f64(dt_err));

        // println!("Framerate: {}", 1.0 / dt);

        prev_time = time;
    }
}

fn statechanged(state: &ServerState, debugtimer: &mut Timer) {
    debugtimer.full_reset();

    match state {
        ServerState::Startup => {

        },
        ServerState::Connected => {
            
        },
        ServerState::Close => {
            
        },
    }
}