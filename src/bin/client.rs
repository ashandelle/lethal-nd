use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use renet_netcode::{ClientAuthentication, NetcodeClientTransport};

use std::{net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::{Duration, SystemTime}};

use lethallib::{client::{ClientSettings, ClientState}, language::Language};
use macroquad::{prelude::*, ui::{Skin, hash, root_ui, widgets::InputText}};

#[macroquad::main("Lethal4D")]
async fn main() {
    let mut prev_time = get_time();

    let mut lang = Language::default();
    let mut settings = ClientSettings::default();
    let mut state: ClientState = ClientState::MainMenu;

    let title_style = root_ui().style_builder()
        // .font(&font).unwrap()
        .text_color(WHITE)
        .font_size(60)
        .build();

    let large_button_style = root_ui().style_builder()
        // .background(button_background)
        // .background_clicked(button_clicked_background)
        // .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 16.0, 8.0, 8.0))
        // .font(&font).unwrap()
        .color(DARKGRAY)
        .color_hovered(GRAY)
        .text_color(WHITE)
        .font_size(30)
        .build();

    let small_button_style = root_ui().style_builder()
        // .background(button_background)
        // .background_clicked(button_clicked_background)
        // .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(8.0, 8.0, 4.0, 4.0))
        // .font(&font).unwrap()
        .color(DARKGRAY)
        .color_hovered(GRAY)
        .text_color(WHITE)
        .font_size(20)
        .build();

    let input_style = root_ui().style_builder()
        // .background(button_background)
        // .background_clicked(button_clicked_background)
        // .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(8.0, 8.0, 4.0, 4.0))
        // .font(&font).unwrap()
        .color(GRAY)
        .text_color(WHITE)
        .font_size(20)
        .build();

    let mut clientoption: Option<RenetClient> = None;
    let mut transportoption: Option<NetcodeClientTransport> = None;

    loop {
        let time = get_time();
        let dt = time - prev_time;

        let width = screen_width();
        let height = screen_height();
        // let ratio = width / height;
        let screen = Vec2::new(width, height);

        match state {
            ClientState::MainMenu => {
                clientoption = None;
                transportoption = None;

                clear_background(LIGHTGRAY);

                let title_skin = Skin {
                    label_style: title_style.clone(),
                    ..root_ui().default_skin()
                };

                let button_skin = Skin {
                    button_style: large_button_style.clone(),
                    label_style: large_button_style.clone(),
                    ..root_ui().default_skin()
                };

                let spacing: f32 = 10.0;

                root_ui().push_skin(&title_skin);

                let title_size = root_ui().calc_size(lang.title);

                root_ui().pop_skin();
                root_ui().push_skin(&button_skin);

                let play_size = root_ui().calc_size(lang.play);
                let settings_size = root_ui().calc_size(lang.settings);
                let exit_size = root_ui().calc_size(lang.exit);

                let vert = title_size.y + spacing + play_size.y + spacing + settings_size.y + spacing + exit_size.y;
                let mut curr = vert / 2.0;

                curr -= exit_size.y / 2.0;
                if root_ui().button((screen - exit_size) / 2.0 + Vec2::new(0.0,curr), lang.exit) {
                    state = ClientState::Exit;
                    printstate(&state);
                }
                curr -= spacing + (exit_size.y + settings_size.y) / 2.0;
                if root_ui().button((screen - settings_size) / 2.0 + Vec2::new(0.0,curr), lang.settings) {
                    state = ClientState::MainSettings;
                    printstate(&state);
                }
                curr -= spacing + (settings_size.y + play_size.y) / 2.0;
                if root_ui().button((screen - play_size) / 2.0 + Vec2::new(0.0,curr), lang.play) {
                    state = ClientState::JoinMenu{
                        address: Default::default(),
                        port: "5000".to_string(),
                    };
                    printstate(&state);
                }

                root_ui().pop_skin();
                root_ui().push_skin(&title_skin);

                curr -= spacing + (play_size.y + title_size.y) / 2.0;
                root_ui().label((screen - title_size) / 2.0 + Vec2::new(0.0,curr), lang.title);

                root_ui().pop_skin();
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

                clear_background(LIGHTGRAY);

                let large_button_skin = Skin {
                    button_style: large_button_style.clone(),
                    label_style: large_button_style.clone(),
                    ..root_ui().default_skin()
                };

                let small_button_skin = Skin {
                    button_style: small_button_style.clone(),
                    label_style: small_button_style.clone(),
                    ..root_ui().default_skin()
                };

                let input_skin = Skin {
                    label_style: input_style.clone(),
                    ..root_ui().default_skin()
                };

                let spacing: f32 = 10.0;
                let address_width: f32 = 150.0;
                let port_width: f32 = 50.0;

                root_ui().push_skin(&large_button_skin);

                let back_size = root_ui().calc_size(lang.back);
                if root_ui().button(Vec2::new(spacing, height - (spacing + back_size.y)), lang.back) {
                    state = ClientState::MainMenu;
                    printstate(&state);
                    break 'JoinMenu;
                }

                root_ui().pop_skin();
                root_ui().push_skin(&small_button_skin);

                let join_size = root_ui().calc_size(lang.join);

                root_ui().pop_skin();
                root_ui().push_skin(&input_skin);

                let address_size = Vec2::new(address_width, join_size.y);
                InputText::new(hash!())
                // .label(lang.address)
                .position((screen - address_size - Vec2::new(0.0, spacing)) / 2.0)
                .size(address_size)
                .ui(&mut root_ui(), address);

                let port_size = Vec2::new(port_width, join_size.y);
                InputText::new(hash!())
                // .label(lang.port)
                .position((screen - address_size + Vec2::new(0.0, spacing)) / 2.0 + Vec2::new(0.0, join_size.y))
                .size(port_size)
                .ui(&mut root_ui(), port);

                root_ui().pop_skin();
                root_ui().push_skin(&small_button_skin);

                if root_ui().button((screen - address_size + Vec2::new(0.0, spacing)) / 2.0 + Vec2::new(address_size.x - join_size.x, join_size.y), lang.join) {
                    // state = ClientState::MainMenu;
                    let addr: Ipv4Addr = match address.parse() {
                        Ok(addr) => addr,
                        Err(err) => {
                            state = ClientState::Disconnected {
                                reason: err.to_string()
                            };
                            printstate(&state);
                            break 'JoinMenu;
                        },
                    };
                    let pt: u16 = match port.parse() {
                        Ok(port) => port,
                        Err(err) => {
                            state = ClientState::Disconnected {
                                reason: err.to_string()
                            };
                            printstate(&state);
                            break 'JoinMenu;
                        },
                    };

                    let socket: SocketAddr = SocketAddr::new(IpAddr::V4(addr), pt);

                    state = ClientState::Connecting { address: socket };
                    printstate(&state);
                }

                root_ui().pop_skin();
            },
            ClientState::Connecting { address } => {
                match (clientoption.as_mut(), transportoption.as_mut()) {
                    (Some(client), Some(transport)) => {
                        let delta_time = Duration::from_secs_f64(dt); // Duration::from_millis(16);
                        // Receive new messages and update client
                        client.update(delta_time);
                        match transport.update(delta_time, client) {
                            Ok(_) => {},
                            Err(error) => {
                                client.disconnect();
                                state = ClientState::Disconnected { reason: error.to_string() };
                                printstate(&state);
                            },
                        };

                        if client.is_disconnected()  {
                            state = ClientState::Disconnected { reason: format!("{:?}", client.disconnect_reason()) };
                            printstate(&state);
                            // break 'Connecting;
                        } else if client.is_connected()  {
                            state = ClientState::Lobby{ lobbyinfo: Default::default() };
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
            ClientState::Lobby { ref mut lobbyinfo } => {
                let client = clientoption.as_mut().unwrap();
                let transport = transportoption.as_mut().unwrap();

                let delta_time = Duration::from_secs_f64(dt);
                // Receive new messages and update client
                client.update(delta_time);
                match transport.update(delta_time, client) {
                    Ok(_) => {},
                    Err(error) => {
                        client.disconnect();
                        state = ClientState::Disconnected { reason: error.to_string() };
                        printstate(&state);
                    },
                };
                
                if client.is_connected() {
                    // Receive message from server
                    while let Some(_message) = client.receive_message(DefaultChannel::ReliableOrdered) {
                        // Handle received message
                    }
                    
                    // Send message
                    client.send_message(DefaultChannel::ReliableOrdered, "client text");
            
                    // Send packets to server using the transport layer
                    let _ = transport.send_packets(client);

                } else if client.is_disconnected() {
                    state = ClientState::Disconnected { reason: format!("{:?}", client.disconnect_reason()) };
                    printstate(&state);
                    // break 'Lobby;
                } else {
                    client.disconnect();
                    state = ClientState::Disconnected { reason: "??".to_string() };
                    printstate(&state);
                    // break 'Lobby;
                }
            },
            ClientState::InGame {} => {
                let client = clientoption.as_mut().unwrap();
                let transport = transportoption.as_mut().unwrap();

            },
            ClientState::Exit => {
                break;
            },
        }
                    
        // std::thread::sleep(delta_time); // Running at 60hz

        // clear_background(LIGHTGRAY);

        // println!("Framerate: {}", 1.0 / dt);

        next_frame().await;

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
        ClientState::Lobby { lobbyinfo: _ } => {
            println!("Connected");
        },
        ClientState::InGame {  } => {
            println!("Game started");
        },
        ClientState::Exit => {
            println!("Exiting");
        },
    }
}