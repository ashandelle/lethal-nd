use std::f64::consts::FRAC_PI_2;

use mathnd::{bivecn::BiVecN, vecn::VecN};

use crate::{BiVecf64, Vecf64, client::{ReliableClientMessage, UnreliableClientMessage, UserInput}, quantize, server::{ReliableServerMessage, UnreliableServerMessage}, world::{entity::Entity, world::{ClientServer, World}}};

impl<'a, const N: usize> World<N> where [(); N - 1]: Sized {
    pub fn client_set_channels(&mut self,
        reliablemessageschannel: Vec<ReliableClientMessage<N>>,
        unreliablemessageschannel: Vec<UnreliableClientMessage<N>>
    ) {
        match &mut self.clientserver {
            ClientServer::Client { reliablemessages, unreliablemessages, .. } => {
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

    pub fn process_reliable_server_messages(&mut self, messages: Vec<ReliableServerMessage<N>>) {
        if let ClientServer::Client {
            reliablemessages: Some(ref mut reliablemessages),
            unreliablemessages: Some(ref mut unreliablemessages),
            ..
        } = self.clientserver {
            for message in messages {
                match message {
                    ReliableServerMessage::ClientConnected { id } => {
                        self.players.insert(id);
                    },
                    ReliableServerMessage::ClientIntro { clientlist } => {
                        self.players.extend(clientlist);
                    },
                    ReliableServerMessage::ClientDisconnected { id, reason } => {
                        self.players.remove(&id);
                        self.entities.remove(&id);
                    },
                    ReliableServerMessage::EntityCreated { id, entitytype, position, rotation } => {
                        self.entities.insert(id, Entity {
                            id,
                            entitytype,
                            position: quantize::f32arr_to_vecn(position),
                            rotation: quantize::u16arr_to_rotation(rotation),
                        });
                    },
                    ReliableServerMessage::EntityDestroyed { id } => {
                        self.entities.remove(&id);
                    }
                }
            }
        } else {
            panic!();
        }
    }

    pub fn process_unreliable_server_messages(&mut self, messages: Vec<UnreliableServerMessage<N>>) {
        if let ClientServer::Client {
            reliablemessages: Some(ref mut reliablemessages),
            unreliablemessages: Some(ref mut unreliablemessages),
            ..
        } = self.clientserver {
            for message in messages {
                match message {
                    UnreliableServerMessage::EntityMoved { id, position, rotation } => {
                        if let Some(entity) = self.entities.get_mut(&id) {
                            (*entity).position = quantize::f32arr_to_vecn(position);
                            (*entity).rotation = quantize::u16arr_to_rotation(rotation);

                            // println!("entity moved");
                        }
                    },
                }
            }
        } else {
            panic!();
        }
    }

    pub fn client_update(&mut self, dt: f64, input: UserInput<N>, mousegrabbed: bool) {
        if let ClientServer::Client {
            id,
            reliablemessages: Some(ref mut reliablemessages),
            unreliablemessages: Some(ref mut unreliablemessages),
            ..
        } = self.clientserver {

            if mousegrabbed {
                let mut moved = false;

                let movementvector: Vecf64<N> = VecN::new(std::array::from_fn(|i| {
                    let val = match (i, input.movementkeys[(i as isize - 1).max(0) as usize]) {
                        (0, _) => 0.0,
                        (_, (true, false)) => 1.0,
                        (_, (false, true)) => -1.0,
                        (_, (_, _)) => 0.0,
                    };
                    if val != 0.0 { moved = true; }
                    val
                }));

                moved |= input.look.0.2 != 0.0;
                moved |= input.look.1.2 != 0.0;

                let mut lookbivector: BiVecf64<N> =
                    BiVecN::basis(input.look.0.0, input.look.0.1) * input.look.0.2 as f64 +
                    BiVecN::basis(input.look.1.0, input.look.1.1) * input.look.1.2 as f64;
                
                if let Some(scrolly) = input.look.2 {
                    lookbivector += BiVecN::basis(scrolly.0, scrolly.1) * scrolly.2 as f64;
                    moved |= scrolly.2 != 0.0;
                }
                
                if let Some(scrollx) = input.look.2 {
                    lookbivector += BiVecN::basis(scrollx.0, scrollx.1) * scrollx.2 as f64;
                    moved |= scrollx.2 != 0.0;
                }

                if moved {
                    if let Some(player) = self.entities.get_mut(&id) {
                        (*player).position += player.player_to_world_norm(movementvector) * dt;
                        (*player).rotation = (*player).rotation.rotate_by_bivector(lookbivector * dt, 1e-8);
                        (*player).rotation.vertical = (*player).rotation.vertical.clamp(-FRAC_PI_2, FRAC_PI_2);

                        unreliablemessages.push(
                            UnreliableClientMessage::PlayerMoved {
                                position: quantize::vecn_to_f32arr(player.position),
                                rotation: quantize::rotation_to_u16arr(player.rotation),
                            }
                        );
                    }
                }
            }
            
            self.update(dt);

        } else {
            panic!();
        }
    }
}