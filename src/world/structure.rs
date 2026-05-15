use bincode::{Decode, Encode};

use crate::client::RenderObject;

#[derive(Encode, Decode, Debug, Clone, Copy)]
pub struct Structure<const N: usize> where [(); N - 1]: Sized {
    pub render: RenderObject<N>,
    // pub hitbox: HitBox<N>,
}