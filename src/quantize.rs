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

use std::f64::consts::FRAC_PI_2;

use mathnd::{matn::MatN, vecn::VecN};

use crate::world::rotation::Rotation;

fn map(val: f64, min1: f64, max1: f64, min2: f64, max2: f64) -> f64 {
    (max2 - min2) * ((val - min1) / (max1 - min1)) + min2
}

pub fn matn_to_f32arr<const N: usize>(mat: MatN<f64, N>) -> [[f32; N]; N] {
    std::array::from_fn(|i|
        std::array::from_fn(|j| mat.e[i].e[j] as f32)
    )
}

pub fn f32arr_to_matn<const N: usize>(arr: [[f32; N]; N]) -> MatN<f64, N> {
    MatN::new(std::array::from_fn(|i|
        VecN::new(std::array::from_fn(|j| arr[i][j] as f64))
    ))
}

pub fn matn_to_u16arr<const N: usize>(mat: MatN<f64, N>) -> ((f32, f32), [[u16; N]; N]) {
    let min = mat.e.iter().map(|v|
        v.e.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    ).fold(f64::INFINITY, |a, b| a.min(b));
    let max = mat.e.iter().map(|v|
        v.e.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    ).fold(f64::NEG_INFINITY, |a, b| a.max(b));

    (
        (min as f32, max as f32),
        std::array::from_fn(|i|
            std::array::from_fn(|j|
                map(mat.e[i].e[j], min, max, 0.0, u16::MAX as f64).round() as u16
            )
        )
    )
}

pub fn u16arr_to_matn<const N: usize>(arr: ((f32, f32), [[u16; N]; N])) -> MatN<f64, N> {
    let ((min, max), arr) = arr;

    MatN::new(std::array::from_fn(|i|
        VecN::new(std::array::from_fn(|j|
            map(arr[i][j] as f64, 0.0, u16::MAX as f64, min as f64, max as f64)
        ))
    ))
}

pub fn vecn_to_f32arr<const N: usize>(vec: VecN<f64, N>) -> [f32; N] {
    std::array::from_fn(|i| vec.e[i] as f32)
}

pub fn f32arr_to_vecn<const N: usize>(arr: [f32; N]) -> VecN<f64, N> {
    VecN::new(std::array::from_fn(|i| arr[i] as f64))
}

pub fn vecn_to_u16arr<const N: usize>(vec: VecN<f64, N>) -> ((f32, f32), [u16; N]) {
    let min = vec.e.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = vec.e.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    (
        (min as f32, max as f32),
        std::array::from_fn(|i|
            map(vec.e[i], min, max, 0.0, u16::MAX as f64).round() as u16
        )
    )
}

pub fn u16arr_to_vecn<const N: usize>(arr: ((f32, f32), [u16; N])) -> VecN<f64, N> {
    let ((min, max), arr) = arr;

    VecN::new(std::array::from_fn(|i|
        map(arr[i] as f64, 0.0, u16::MAX as f64, min as f64, max as f64)
    ))
}

pub fn rotation_to_u16arr<const N: usize>(rot: Rotation<f64, N>) -> ((f32, f32), [[u16; N-1]; N-1], u16) {
    let (minmax, arr) = matn_to_u16arr(rot.horizontal);
    let vert = map(rot.vertical, -FRAC_PI_2, FRAC_PI_2, 0.0, u16::MAX as f64).round() as u16;

    (minmax, arr, vert)
}

pub fn u16arr_to_rotation<const N: usize>(arr: ((f32, f32), [[u16; N-1]; N-1], u16)) -> Rotation<f64, N> {
    let (minmax, arr, vert) = arr;
    let mat = u16arr_to_matn((minmax, arr)).orthonormalized(1e-8, 128);
    let vert = map(vert as f64, 0.0, u16::MAX as f64, -FRAC_PI_2, FRAC_PI_2);

    Rotation { horizontal: mat, vertical: vert }
}