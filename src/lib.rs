#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// use crate::math::{Vec4};
pub mod math;
pub mod player;
pub mod rotation;
pub mod quantize;
pub mod language;
pub mod lobbydata;

pub mod server;
pub mod client;

// pub mod serverstate;
// pub mod servermessage;
// pub mod serversettings;

// pub mod clientstate;
// pub mod clientmessage;
// pub mod clientsettings;

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
