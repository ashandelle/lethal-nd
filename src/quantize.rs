// use std::f32::consts::PI;

// use mathnd::{matn::MatN, vecn::VecN};

// use crate::rotation::{Rotation, RotationQuantized};

// const NORMALIZED_U16_MAX: f32 = 65535 as f32;

// impl<const N: usize> VecN<f32, N> {
//     pub fn quantize_normalized_u16(&self) -> VecN<u16, N> {
//         VecN::new(std::array::from_fn(|i|
//             (NORMALIZED_U16_MAX * ((self.e[i] / 2.0) + 0.5)).round().clamp(u16::MIN as f32, u16::MAX as f32) as u16
//         ))
//     }
// }

// impl<const N: usize> VecN<u16, N> {
//     pub fn unquantize_normalized_u16(&self) -> VecN<f32, N> {
//         VecN::new(std::array::from_fn(|i|
//             (((self.e[i] as f32) / NORMALIZED_U16_MAX) - 0.5) * 2.0
//         ))
//     }
// }

// impl<const N: usize> MatN<f32, N> {
//     pub fn quantize_normalized_u16(&self) -> MatN<u16, N> {
//         MatN::new(std::array::from_fn(|i|
//             self.e[i].quantize_normalized_u16()
//         ))
//     }
// }

// impl<const N: usize> MatN<u16, N> {
//     pub fn unquantize_normalized_u16(&self) -> MatN<f32, N> {
//         MatN::new(std::array::from_fn(|i|
//             self.e[i].unquantize_normalized_u16()
//         ))
//     }
// }

// const ROTATION_MAX: f32 = 255 as f32;

// impl<const N: usize> Rotation<f32, N> where [(); N - 1]: Sized {
//     pub fn quantize(&self) -> RotationQuantized<N> {
//         RotationQuantized {
//             vertical: (ROTATION_MAX * ((self.vertical / PI) + 0.5)).round().clamp(u8::MIN as f32, u8::MAX as f32) as u8,
//             horizontal: self.horizontal.quantize_normalized_u16(),
//         }
//     }
// }

// impl<const N: usize> RotationQuantized<N> where [(); N - 1]: Sized {
//     pub fn unquantize(&self) -> Rotation<f32, N> {
//         Rotation {
//             vertical: (((self.vertical as f32) / ROTATION_MAX) - 0.5) * PI,
//             horizontal: self.horizontal.unquantize_normalized_u16(),
//         }
//     }
// }