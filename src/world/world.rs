use std::collections::HashMap;

use crate::{server::{ReliableServerMessage, UnreliableServerMessage}, world::entity::Entity};

pub struct World<const N: usize> where [(); N - 1]: Sized {
    pub entities: HashMap<u64, Entity<N>>,
}

// Client
impl<const N: usize> World<N> where [(); N - 1]: Sized {
    // pub fn process_user_input(id: u64, input) {

    // }

    pub fn process_reliable_server_messages(messages: Vec<ReliableServerMessage<N>>) {

    }

    pub fn process_unreliable_server_messages(messages: Vec<UnreliableServerMessage<N>>) {

    }
}

// Server
// impl<const N: usize> World<N> where [(); N - 1]: Sized {
//     pub fn process_reliable_client_messages(messages: Vec<ReliableClientMessage>) {

//     }

//     pub fn process_unreliable_client_messages(messages: Vec<UnreliableClientMessage>) {

//     }
// }

impl<const N: usize> World<N> where [(); N - 1]: Sized {
    pub fn new() -> Self {
        World {
            entities: HashMap::new(),
        }
    }
}