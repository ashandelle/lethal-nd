use bincode::error::DecodeError;
use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use renet_netcode::{ClientAuthentication, NetcodeClientTransport};

use std::{net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::{Duration, SystemTime, UNIX_EPOCH}};

use lethallib::{client::{ClientConnectedState, ClientSettings, ClientState, ReliableClientMessage, UnreliableClientMessage}, disconnected_menu, join_menu, language::Language, main_menu, server::{self, ReliableServerMessage, UnreliableServerMessage}, skins, styles};
use macroquad::{prelude::*, ui::{Skin, hash, root_ui, widgets::InputText}};

macro_rules! client_update {
    ($dt:ident, $client:ident, $transport:ident, $state:ident) => {
        let delta_time = Duration::from_secs_f64($dt); // Duration::from_millis(16);
        // Receive new messages and update client
        $client.update(delta_time);
        match $transport.update(delta_time, $client) {
            Ok(_) => {},
            Err(error) => {
                $client.disconnect();
                $state = ClientState::Disconnected { reason: error.to_string() };
                printstate(&$state);
            },
        };
    }
}

#[macroquad::main("Lethal4D")]
async fn main() {
    const N: usize = 3;

    let target_fps = 60;
    let target_dt = 1.0 / target_fps as f64;
    let mut dt_err = 0.0;

    let mut prev_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time should go forward");

    let mut lang = Language::default();
    let mut settings = ClientSettings::default();
    let mut state: ClientState = ClientState::MainMenu;

    styles!(title_style, large_button_style, small_button_style, input_style);
    skins!(
        title_style, large_button_style, small_button_style, input_style,
        title_skin, large_button_skin, small_button_skin, input_skin
    );

    let mut clientoption: Option<RenetClient> = None;
    let mut transportoption: Option<NetcodeClientTransport> = None;

    loop {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time should go forward");
        let dt = (time - prev_time).as_secs_f64();

        let width = screen_width();
        let height = screen_height();
        // let ratio = width / height;
        let screen = Vec2::new(width, height);

        match state {
            ClientState::MainMenu => {
                clientoption = None;
                transportoption = None;

                main_menu!(lang, state, screen, title_skin, large_button_skin);
            },
            ClientState::MainSettings => {
                clientoption = None;
                transportoption = None;

                clear_background(LIGHTGRAY);

                let button_skin = Skin {
                    button_style: large_button_style.clone(),
                    label_style: large_button_style.clone(),
                    ..root_ui().default_skin()
                };

                let spacing: f32 = 10.0;

                root_ui().push_skin(&button_skin);

                let back_size = root_ui().calc_size(lang.back);
                if root_ui().button(Vec2::new(spacing, height - (spacing + back_size.y)), lang.back) {
                    state = ClientState::MainMenu;
                    printstate(&state);
                }

                root_ui().pop_skin();
            },
            ClientState::JoinMenu { ref mut address, ref mut port } => 'JoinMenu: {
                clientoption = None;
                transportoption = None;

                join_menu!(lang, 'JoinMenu, state, address, port, height, screen, large_button_skin, small_button_skin, input_skin);
            },
            ClientState::Connecting { address } => {
                match (clientoption.as_mut(), transportoption.as_mut()) {
                    (Some(client), Some(transport)) => {
                        client_update!(dt, client, transport, state);

                        if client.is_disconnected()  {
                            state = ClientState::Disconnected { reason: format!("{:?}", client.disconnect_reason()) };
                            printstate(&state);
                            // break 'Connecting;
                        } else if client.is_connected()  {
                            state = ClientState::Connected { connectedstate: ClientConnectedState::Lobby };
                            printstate(&state);
                            // break 'Connecting;
                        }
                    },
                    (_client, _transport) => {
                        let client = RenetClient::new(ConnectionConfig::default());

                        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

                        // Setup transport layer using renet_netcode
                        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                        let time: u128 = current_time.as_nanos();
                        let id = (time >> 64) as u64 ^ (time & u64::MAX as u128) as u64;

                        let authentication = ClientAuthentication::Unsecure {
                            server_addr: address,
                            client_id: id,
                            user_data: None,
                            protocol_id: 0,
                        };

                        println!("Id {}", id);

                        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

                        clientoption = Some(client);
                        transportoption = Some(transport);
                    },
                }
            },
            ClientState::Disconnected { ref reason } => {
                clientoption = None;
                transportoption = None;

                disconnected_menu!(lang, state, reason, height, screen, large_button_skin);
            },
            ClientState::Connected { ref mut connectedstate } => {
                macro_rules! receive_messages {
                    ($client:ident, $messages_name:ident, $message_type:ty, $channel_id:expr) => {
                        let mut $messages_name: Vec<$message_type> = Vec::new();

                        // Receive message from server
                        while let Some(message) = $client.receive_message($channel_id) {
                            let message: Result<($message_type, usize), DecodeError>  = bincode::decode_from_slice(&message, bincode::config::standard());

                            match message {
                                Ok((servermessage, _)) => {
                                    $messages_name.push(servermessage);
                                },
                                Err(err) => {
                                    println!("Error: {:?}", err);
                                }
                            }
                        }
                    }
                }

                macro_rules! send_messages {
                    ($client:ident, $messages_name:ident, $message_type:ty, $channel_id:expr) => {
                        for message in $messages_name.iter() {
                            let message = bincode::encode_to_vec(&message, bincode::config::standard()).unwrap();
                            $client.send_message($channel_id, message);
                        }
                    };
                }

                let client = clientoption.as_mut().unwrap();
                let transport = transportoption.as_mut().unwrap();

                client_update!(dt, client, transport, state);
                
                if client.is_connected() {
                    let mut reliablemessagessent: Vec<ReliableClientMessage<N>> = Vec::new();
                    let mut unreliablemessagessent: Vec<UnreliableClientMessage<N>> = Vec::new();

                    receive_messages!(client, reliablemessagesreceived, ReliableServerMessage<N>, DefaultChannel::ReliableOrdered);
                    receive_messages!(client, unreliablemessagesreceived, UnreliableServerMessage<N>, DefaultChannel::Unreliable);
                    





                    
                    send_messages!(client, reliablemessagessent, ReliableServerMessage<N>, DefaultChannel::ReliableOrdered);
                    send_messages!(client, unreliablemessagessent, UnreliableServerMessage<N>, DefaultChannel::Unreliable);
            
                    // Send packets to server using the transport layer
                    match transport.send_packets(client) {
                        Ok(_) => {},
                        Err(err) => {
                            println!("Error: {:?}", err);
                        },
                    }

                } else if client.is_disconnected() {
                    state = ClientState::Disconnected { reason: format!("{:?}", client.disconnect_reason()) };
                    printstate(&state);
                } else {
                    client.disconnect();
                    state = ClientState::Disconnected { reason: "??".to_string() };
                    printstate(&state);
                }
            },
            ClientState::Exit => {
                break;
            },
        }

        // clear_background(LIGHTGRAY);

        next_frame().await;

        dt_err += target_dt - dt;
        dt_err = dt_err.max(0.0);
        std::thread::sleep(Duration::from_secs_f64(dt_err));

        println!("Framerate: {}", 1.0 / dt);

        prev_time = time;
    }
}

fn printstate(state: &ClientState) {
    match state {
        ClientState::MainMenu => {

        },
        ClientState::MainSettings => {
            
        },
        ClientState::JoinMenu { address: _, port: _ } => {
            
        },
        ClientState::Connecting { address: _ } => {
            println!("Connecting...");
        },
        ClientState::Disconnected { reason } => {
            println!("Disconnected: {}", reason);
        },
        ClientState::Connected { connectedstate } => {
            println!("Connected");

            match connectedstate {
                ClientConnectedState::Lobby => {
                    println!("In lobby");
                },
                ClientConnectedState::InGame => {
                    println!("Game started");
                },
            }
        },
        ClientState::Exit => {
            println!("Exiting");
        },
    }
}