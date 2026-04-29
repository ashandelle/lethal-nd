use bincode::{Decode, Encode};

use crate::{disconnectreason::DisconnectReason, world::entity::Entity};
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
        // entity: Entity<N>,
    },
    EntityDestroyed {
        id: u64,
    },
}

#[derive(Encode, Decode, Debug)]
pub enum UnreliableServerMessage<const N: usize> where [(); N - 1]: Sized {
    EntityMoved {
        id: u64,
        // position: ,
        // rotation: ,
    },
}