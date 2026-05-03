use std::net::SocketAddr;

#[derive(Debug)]
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