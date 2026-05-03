#[derive(Debug)]
pub enum ServerState {
    Startup,
    Connected,
    Close,
}