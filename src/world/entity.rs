use bincode::{Decode, Encode};

use crate::{Vecf64, Rotationf64};

#[derive(Debug)]
pub struct Entity<const N: usize> where [(); N - 1]: Sized {
    pub id: u64,
    pub entitytype: EntityType,

    pub position: Vecf64<N>,
    // pub velocity: Vecf64<N>,
    pub rotation: Rotationf64<N>,
}

#[derive(Encode, Decode, Debug, Clone, Copy)]
pub enum EntityType {
    Player,
}

impl<const N: usize> Entity<N> where [(); N - 1]: Sized {
    pub fn world_to_player_vec(&self, vec: Vecf64<N>) -> Vecf64<N> {
        self.rotation.rotate_horizontal(vec - self.position)
    }
    pub fn world_to_player_norm(&self, vec: Vecf64<N>) -> Vecf64<N> {
        self.rotation.rotate_horizontal(vec)
    }

    pub fn player_to_world_vec(&self, vec: Vecf64<N>) -> Vecf64<N> {
        self.rotation.transpose_rotate_horizontal(vec) + self.position
    }
    pub fn player_to_world_norm(&self, vec: Vecf64<N>) -> Vecf64<N> {
        self.rotation.transpose_rotate_horizontal(vec)
    }

    pub fn world_to_camera_vec(&self, vec: Vecf64<N>) -> Vecf64<N> {
        self.rotation.rotate(vec - self.position)
    }
    pub fn world_to_camera_norm(&self, vec: Vecf64<N>) -> Vecf64<N> {
        self.rotation.rotate(vec)
    }

    pub fn camera_to_world_vec(&self, vec: Vecf64<N>) -> Vecf64<N> {
        self.rotation.transpose_rotate(vec) + self.position
    }
    pub fn camera_to_world_norm(&self, vec: Vecf64<N>) -> Vecf64<N> {
        self.rotation.transpose_rotate(vec)
    }
}