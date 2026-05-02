use std::collections::HashMap;

use crate::{client::{ReliableClientMessage, UnreliableClientMessage}, server::{ReliableServerMessage, ServerMessageVisibility, UnreliableServerMessage}, world::entity::Entity};

pub struct World<const N: usize> where [(); N - 1]: Sized {
    pub entities: HashMap<u64, Entity<N>>,
    pub clientserver: ClientServer<N>,
}

pub enum ClientServer<const N: usize> where [(); N - 1]: Sized {
    Client {
        reliablemessages: Option<Vec<ReliableClientMessage<N>>>,
        unreliablemessages: Option<Vec<UnreliableClientMessage<N>>>,
    },
    Server {
        reliablemessages: Option<Vec<(ServerMessageVisibility, ReliableServerMessage<N>)>>,
        unreliablemessages: Option<Vec<(ServerMessageVisibility, UnreliableServerMessage<N>)>>,
    }
}

impl<const N: usize> World<N> where [(); N - 1]: Sized {
    pub fn new_client() -> Self {
        World {
            entities: HashMap::new(),
            clientserver: ClientServer::Client { reliablemessages: None, unreliablemessages: None }
        }
    }

    pub fn new_server() -> Self {
        World {
            entities: HashMap::new(),
            clientserver: ClientServer::Server { reliablemessages: None, unreliablemessages: None }
        }
    }

    pub fn update(&mut self, dt: f64) {
        
    }
}