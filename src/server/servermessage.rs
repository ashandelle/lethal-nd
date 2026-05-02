use bincode::{Decode, Encode};

use crate::{disconnectreason::DisconnectReason, world::entity::EntityType};
// use serde::{Deserialize, Serialize};

pub enum ServerMessageVisibility {
    Broadcast,
    Except {
        id: u64
    },
    Only {
        id: u64
    },
}

#[derive(Encode, Decode, Debug)]
pub enum ReliableServerMessage<const N: usize> where [(); N - 1]: Sized {
    ClientConnected {
        id: u64,
    },
    ClientDisconnected {
        id: u64,
        reason: DisconnectReason,
    },
    EntityCreated {
        id: u64,
        entitytype: EntityType,
        position: [f32; N],
        rotation: ((f32, f32), [[u16; N-1]; N-1], u16),
    },
    EntityDestroyed {
        id: u64,
    },
}

#[derive(Encode, Decode, Debug)]
pub enum UnreliableServerMessage<const N: usize> where [(); N - 1]: Sized {
    EntityMoved {
        id: u64,
        position: [f32; N],
        rotation: ((f32, f32), [[u16; N-1]; N-1], u16),
    },
}