use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug)]
pub enum ReliableClientMessage<const N: usize> where [(); N - 1]: Sized {
    //
}

#[derive(Encode, Decode, Debug)]
pub enum UnreliableClientMessage<const N: usize> where [(); N - 1]: Sized {
    PlayerMoved {
        // position: ,
        // rotation: ,
    },
}