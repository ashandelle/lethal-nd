use crate::{client::{ReliableClientMessage, UnreliableClientMessage}, server::{ReliableServerMessage, UnreliableServerMessage}, world::world::{ClientServer, World}};

impl<'a, const N: usize> World<N> where [(); N - 1]: Sized {
    pub fn client_set_channels(&mut self,
        reliablemessageschannel: Vec<ReliableClientMessage<N>>,
        unreliablemessageschannel: Vec<UnreliableClientMessage<N>>
    ) {
        match &mut self.clientserver {
            ClientServer::Client { reliablemessages, unreliablemessages } => {
                *reliablemessages = Some(reliablemessageschannel);
                *unreliablemessages = Some(unreliablemessageschannel);
            },
            _ => panic!(),
        }
    }

    pub fn client_extract_channels(&mut self) -> (
        Vec<ReliableClientMessage<N>>,
        Vec<UnreliableClientMessage<N>>
    ) {
        if let ClientServer::Client {
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

    // pub fn process_user_input(&mut self, id: u64, input) {

    // }

    pub fn process_reliable_server_messages(&mut self, messages: Vec<ReliableServerMessage<N>>) {

    }

    pub fn process_unreliable_server_messages(&mut self, messages: Vec<UnreliableServerMessage<N>>) {

    }
}