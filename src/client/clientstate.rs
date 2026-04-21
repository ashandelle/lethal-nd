use std::net::SocketAddr;

use renet::RenetClient;
use renet_netcode::NetcodeClientTransport;

use crate::lobbydata::LobbyData;

pub enum ClientState {
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
        reason: String,
    },
    Lobby {
        lobbyinfo: LobbyData,
    },
    InGame {
    },
    Exit,
}