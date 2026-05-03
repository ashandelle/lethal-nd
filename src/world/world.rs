use std::collections::{HashMap, HashSet};

use derive_debug::Dbg;

use crate::{client::{ReliableClientMessage, UnreliableClientMessage}, server::{ReliableServerMessage, ServerMessageVisibility, UnreliableServerMessage}, world::entity::Entity};

#[derive(Debug)]
pub struct World<const N: usize> where [(); N - 1]: Sized {
    pub entities: HashMap<u64, Entity<N>>,
    pub players: HashSet<u64>,
    pub clientserver: ClientServer<N>,
}

#[derive(Dbg)]
pub enum ClientServer<const N: usize> where [(); N - 1]: Sized {
    #[dbg(skip)]
    Client {
        id: u64,
        // positiontimer: Timer,
        reliablemessages: Option<Vec<ReliableClientMessage<N>>>,
        unreliablemessages: Option<Vec<UnreliableClientMessage<N>>>,
    },
    #[dbg(skip)]
    Server {
        reliablemessages: Option<Vec<(ServerMessageVisibility, ReliableServerMessage<N>)>>,
        unreliablemessages: Option<Vec<(ServerMessageVisibility, UnreliableServerMessage<N>)>>,
    }
}

impl<const N: usize> World<N> where [(); N - 1]: Sized {
    pub fn new_client(id: u64) -> Self {
        World {
            entities: HashMap::new(),
            players: HashSet::new(),
            clientserver: ClientServer::Client { id, reliablemessages: None, unreliablemessages: None }
        }
    }

    pub fn new_server() -> Self {
        World {
            entities: HashMap::new(),
            players: HashSet::new(),
            clientserver: ClientServer::Server { reliablemessages: None, unreliablemessages: None }
        }
    }

    pub fn update(&mut self, dt: f64) {

    }
}