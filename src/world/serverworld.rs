use mathnd::vecn::VecN;

use crate::{client::{ReliableClientMessage, UnreliableClientMessage}, disconnectreason::DisconnectReason, quantize, server::{ReliableServerMessage, ServerMessageVisibility, UnreliableServerMessage}, world::{entity::{Entity, EntityType}, rotation::Rotation, world::{ClientServer, World}}};

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
            ..
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
                ServerMessageVisibility::Only{ id },
                ReliableServerMessage::ClientIntro { clientlist: self.players.clone() }
            ));
            self.players.insert(id);
            reliablemessages.push((
                ServerMessageVisibility::Except { id },
                ReliableServerMessage::ClientConnected { id }
            ));
        } else {
            panic!();
        }

        self.create_player(id);
        
        if let ClientServer::Server {
            reliablemessages: Some(ref mut reliablemessages),
            ..
        } = self.clientserver {
            if let Some(Entity { id, entitytype, position, rotation }) = self.entities.get(&id) {
                reliablemessages.push((
                    ServerMessageVisibility::Except{ id: *id },
                    ReliableServerMessage::EntityCreated {
                        id: *id,
                        entitytype: *entitytype,
                        position: quantize::vecn_to_f32arr(*position),
                        rotation: quantize::rotation_to_u16arr(*rotation),
                    }
                ));
            }

            for (entityid, Entity { id: _, entitytype, position, rotation }) in &self.entities {
                reliablemessages.push((
                    ServerMessageVisibility::Only{ id },
                    ReliableServerMessage::EntityCreated {
                        id: *entityid,
                        entitytype: *entitytype,
                        position: quantize::vecn_to_f32arr(*position),
                        rotation: quantize::rotation_to_u16arr(*rotation),
                    }
                ));
            }
        } else {
            panic!();
        }
    }

    pub fn create_player(&mut self, id: u64) {
        self.entities.insert(id, Entity {
            id,
            entitytype: EntityType::Player,
            position: VecN::zero(),
            rotation: Rotation::identity(),
        });
    }

    pub fn player_disconnected(&mut self, id: u64, reason: DisconnectReason) {
        if let ClientServer::Server {
            reliablemessages: Some(ref mut reliablemessages),
            ..
        } = self.clientserver {
            self.players.remove(&id);
            self.entities.remove(&id);
            reliablemessages.push((
                ServerMessageVisibility::Except { id: id },
                ReliableServerMessage::ClientDisconnected { id: id, reason: reason }
            ));
        } else {
            panic!();
        }
    }

    pub fn process_reliable_client_messages(&mut self, messages: Vec<(u64, ReliableClientMessage<N>)>) {
        if let ClientServer::Server {
            reliablemessages: Some(ref mut reliablemessages),
            unreliablemessages: Some(ref mut unreliablemessages),
            ..
        } = self.clientserver {
            for (id, message) in messages {
                match message {

                }
            }
        } else {
            panic!();
        }
    }

    pub fn process_unreliable_client_messages(&mut self, messages: Vec<(u64, UnreliableClientMessage<N>)>) {
        if let ClientServer::Server {
            reliablemessages: Some(ref mut reliablemessages),
            unreliablemessages: Some(ref mut unreliablemessages),
            ..
        } = self.clientserver {
            for (id, message) in messages {
                match message {
                    UnreliableClientMessage::PlayerMoved { position, rotation } => {
                        if let Some(entity) = self.entities.get_mut(&id) {
                            (*entity).position = quantize::f32arr_to_vecn(position);
                            (*entity).rotation = quantize::u16arr_to_rotation(rotation);

                            // println!("player moved");

                            unreliablemessages.push((
                                ServerMessageVisibility::Except { id: id },
                                UnreliableServerMessage::EntityMoved { id, position, rotation }
                            ));
                        }
                    },
                }
            }
        } else {
            panic!();
        }
    }

    pub fn server_update(&mut self, dt: f64) {
        self.update(dt);
    }
}