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
        client: RenetClient,
        transport: NetcodeClientTransport,
        lobbyinfo: LobbyData,
    },
    InGame {
        client: RenetClient,
        transport: NetcodeClientTransport,
    },
    Exit,
}