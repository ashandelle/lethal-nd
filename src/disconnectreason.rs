use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug)]
pub enum DisconnectReason {
    Left,
    Kicked,
    NetworkError,
}