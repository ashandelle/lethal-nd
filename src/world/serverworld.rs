use crate::{client::{ReliableClientMessage, UnreliableClientMessage}, disconnectreason::DisconnectReason, server::{ReliableServerMessage, ServerMessageVisibility, UnreliableServerMessage}, world::world::{ClientServer, World}};

impl<'a, const N: usize> World<N> where [(); N - 1]: Sized {
    pub fn server_set_channels(&mut self,
        reliablemessageschannel: Vec<(ServerMessageVisibility, ReliableServerMessage<N>)>,
        unreliablemessageschannel: Vec<(ServerMessageVisibility, UnreliableServerMessage<N>)>
    ) {
        match &mut self.clientserver {
            ClientServer::Server { reliablemessages, unreliablemessages } => {
                *reliablemessages = Some(reliablemessageschannel);
                *unreliablemessages = Some(unreliablemessageschannel);
            },
            _ => panic!(),
        }
    }

    pub fn server_extract_channels(&mut self) -> (
        Vec<(ServerMessageVisibility, ReliableServerMessage<N>)>,
        Vec<(ServerMessageVisibility, UnreliableServerMessage<N>)>
    ) {
        if let ClientServer::Server {
            reliablemessages,
            unreliablemessages,
        } = &mut self.clientserver {
            match (reliablemessages.take(), unreliablemessages.take()) {
                (Some(reliable), Some(unreliable)) => (reliable, unreliable),
                _ => panic!(),
            }
        } else {
            panic!();
        }
    }

    pub fn player_connected(&mut self, id: u64) {
        if let ClientServer::Server {
            reliablemessages: Some(ref mut reliablemessages),
            ..
        } = self.clientserver {
            reliablemessages.push((
                ServerMessageVisibility::Except { id: id },
                ReliableServerMessage::ClientConnected { id: id }
            ));
        } else {
            panic!();
        }
    }

    pub fn player_disconnected(&mut self, id: u64, reason: DisconnectReason) {
        if let ClientServer::Server {
            reliablemessages: Some(ref mut reliablemessages),
            ..
        } = self.clientserver {
            reliablemessages.push((
                ServerMessageVisibility::Except { id: id },
                ReliableServerMessage::ClientDisconnected { id: id, reason: reason }
            ));
        } else {
            panic!();
        }
    }

    pub fn process_reliable_client_messages(&mut self, messages: Vec<ReliableClientMessage<N>>) {

    }

    pub fn process_unreliable_client_messages(&mut self, messages: Vec<UnreliableClientMessage<N>>) {

    }
}