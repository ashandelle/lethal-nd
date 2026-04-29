#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use mathnd::{bivecn::BiVecN, matn::MatN, vecn::VecN};

use crate::world::rotation::Rotation;

pub mod quantize;
pub mod language;
pub mod lobbydata;
pub mod disconnectreason;

pub mod server;
pub mod client;
pub mod world;

pub type Vecf64<const N: usize> = VecN<f64, N>;
pub type Matf64<const N: usize> = MatN<f64, N>;
pub type BiVecf64<const N: usize> = BiVecN<f64, N>;
pub type Rotationf64<const N: usize> = Rotation<f64, N>;

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn test_vecn() {
    //     let vec1: Vec4<f64> = Vec4::<f64>::new([1.0, 2.0, 3.0, 4.0]);
    //     let vec2: Vec4<f64> = Vec4::<f64>::new([-4.0, -3.0, -2.0, -1.0]);
    //     assert_eq!(vec1 + vec2, Vec4::<f64>::new([-3.0, -1.0, 1.0, 3.0]));
    // }
}
