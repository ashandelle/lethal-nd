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
    pub fn worldtoplayer(&self, vec: Vecf64<N>) -> Vecf64<N> {
        self.rotation.rotate_horizontal(vec - self.position)
    }

    pub fn playertoworld(&self, vec: Vecf64<N>) -> Vecf64<N> {
        self.rotation.transpose_rotate_horizontal(vec) + self.position
    }

    pub fn worldtocamera(&self, vec: Vecf64<N>) -> Vecf64<N> {
        self.rotation.rotate(vec - self.position)
    }

    pub fn cameratoworld(&self, vec: Vecf64<N>) -> Vecf64<N> {
        self.rotation.transpose_rotate(vec) + self.position
    }
}