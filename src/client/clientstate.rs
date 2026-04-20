use std::net::SocketAddr;

use renet::RenetClient;
use renet_netcode::NetcodeClientTransport;

use crate::lobbydata::LobbyData;

pub enum ClientState<'a> {
    MainMenu,
    MainSettings,
    JoinMenu {
        address: String,
        port: String,
    },
    Connecting {
        address: SocketAddr,
    },
    Disconnected {
        reason: &'a str,
    },
    Lobby {
        lobbyinfo: LobbyData,
    },
    InGame {
    },
    Exit,
}