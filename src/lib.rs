#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use mathnd::{bivecn::BiVecN, matn::MatN, vecn::VecN};

use crate::world::rotation::Rotation;

pub mod quantize;
pub mod language;
pub mod lobbydata;
pub mod disconnectreason;
pub mod timer;
pub mod util;

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
    use mathnd::{matn::MatN, vecn::VecN};
    // use super::*;
    use rand::prelude::*;
    use rand_distr::StandardNormal;

    use crate::world::rotation::Rotation;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn test_rotation() {
        let mut rng = rand::rng();

        for _n in 0..100 {
            let rot: Rotation<f64, 3> = Rotation {
                horizontal: MatN {
                    e: std::array::from_fn(|_i|
                        VecN::new(std::array::from_fn(|_j| rng.sample(StandardNormal)))
                    ),
                }.orthonormalized(1e-8, 128),
                vertical: rng.sample(StandardNormal),
            };
            
            let mut vec: VecN<f64, 3> = VecN::new(std::array::from_fn(|_i| rng.sample(StandardNormal)));
            vec.e[0] = 0.0;
            vec.normalize();

            let vec1 = rot.rotate(vec);
            let vec2 = rot.rotate_horizontal(vec);

            assert!((vec.length() - vec1.length()).abs() < 1e-8);
            assert!((vec.length() - vec2.length()).abs() < 1e-8);

            // println!("{}\t{}\t{}\t{}\t{}", vec1.dot(vec2), rot.vertical, rot.vertical.cos(), vec1.length(), vec2.length());

            // println!("{:?} {:?}", rot, vec);

            // assert!((vec1.dot(vec2) - rot.vertical.cos()).abs() < 1e-8);

            assert!((vec - rot.transpose_rotate(vec1)).length_sqr() < 1e-8);
            assert!((vec - rot.transpose_rotate_horizontal(vec2)).length_sqr() < 1e-8);
        }

        // assert!(false);
    }
}
