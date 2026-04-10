use crate::{math::{Vecf64}, rotation::Rotation};

pub struct Player<const N: usize> where [(); N - 1]: Sized {
    pub position: Vecf64<N>,
    pub velocity: Vecf64<N>,
    pub rotation: Rotation<f64, N>,
}

impl<const N: usize> Player<N> where [(); N - 1]: Sized {
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