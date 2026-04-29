use std::net::SocketAddr;

// use crate::lobbydata::LobbyData;

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
    Connected,
    Exit,
}

// pub enum ClientConnectedState {
//     Lobby,
//     InGame,
// }