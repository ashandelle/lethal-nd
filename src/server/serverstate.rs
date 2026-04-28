pub enum ServerState {
    Startup,
    Connected {
        connectedstate: ServerConnectedState,
    },
    Close,
}

pub enum ServerConnectedState {
    Lobby,
    InGame,
}