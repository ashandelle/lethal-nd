use bincode::{Decode, Encode};

use crate::{disconnectreason::DisconnectReason, world::entity::Entity};
// use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Debug)]
pub enum ServerMessage<const N: usize> where [(); N - 1]: Sized {
    Reliable {
        message: ReliableServerMessage<N>,
    },
    Unreliable {
        message: UnreliableServerMessage<N>,
    },
}

#[derive(Encode, Decode, Debug)]
pub enum ReliableServerMessage<const N: usize> where [(); N - 1]: Sized {
    // CreatePlayer {
    //     entity: Entity,
    //     translation: [f32; 3],
    //     rotation: f32,
    //     name: String,
    // },
    // RemoveEntities {
    //     entities: Vec<Entity>,
    // },
    // Move {
    //     entity: Entity,
    //     translation: [f32; 3],
    //     rotation: f32,
    // },
    // GameStart,
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