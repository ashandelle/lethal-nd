use std::{iter::Sum, ops::{Add, Div, Mul, Neg, Sub}};

use crate::math::{CosSin, One, Sqrt, Zero};

#[derive(Debug, Clone, Copy)]
pub struct VecN<T, const N: usize> {
    pub(crate) e: [T; N]
}

impl<T, const N: usize> PartialEq for VecN<T, N> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.e == other.e
    }
}

impl<T, const N: usize> Neg for VecN<T, N> where T: Neg<Output = T> + Copy {
    type Output = VecN<T, N>;
    fn neg(self) -> VecN<T, N> {
        VecN::new(std::array::from_fn(|i| -self.e[i]))
    }
}

impl<T, const N: usize> Add for VecN<T, N> where T: Add<Output = T> + Copy {
    type Output = VecN<T, N>;
    fn add(self, v: VecN<T, N>) -> VecN<T, N> {
        VecN::new(std::array::from_fn(|i| self.e[i] + v.e[i]))
    }
}

impl<T, const N: usize> Sub for VecN<T, N> where T: Sub<Output = T> + Copy {
    type Output = VecN<T, N>;
    fn sub(self, v: VecN<T, N>) -> VecN<T, N> {
        VecN::new(std::array::from_fn(|i| self.e[i] - v.e[i]))
    }
}

// impl<T, const N: usize> Mul<VecN<T, N>> for T where T: Mul<Output = T> + Copy {
//     type Output = VecN<T, N>;
//     fn mul(self, v: VecN<T, N>) -> VecN<T, N> {
//         VecN::new(std::array::from_fn(|i| self * v.e[i]))
//     }
// }
impl<T, const N: usize> Mul<T> for VecN<T, N> where T: Mul<Output = T> + Copy {
    type Output = VecN<T, N>;
    fn mul(self, s: T) -> VecN<T, N> {
        VecN::new(std::array::from_fn(|i| self.e[i] * s))
    }
}

impl<T, const N: usize> Div<T> for VecN<T, N> where T: Div<Output = T> + Copy {
    type Output = VecN<T, N>;
    fn div(self, s: T) -> VecN<T, N> {
        VecN::new(std::array::from_fn(|i| self.e[i] / s))
    }
}

// impl BitXor for VecN {
//     type Output = BiVecN;
//     fn bitxor(self, v: VecN) -> BiVecN {
//         let mut vecs: Vec<VecN> = Vec::new();
//
//         for i in 0..v.e.len() {
//             let mut vec: Vec<f64> = Vec::new();
//             for j in 0..v.e.len() {
//                 vec.push(
//                     self.e[i] * v.e[j] - self.e[j] * v.e[i]
//                 );
//             }
//             vecs.push(VecN{e: vec});
//         }
//
//         BiVecN {
//             m: MatN{e: vecs},
//         }.skew()
//     }
// }

// impl fmt::Display for VecN {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self.e)
//     }
// }

impl<T, const N: usize> VecN<T, N> {
    pub fn new(e: [T; N]) -> Self {
        VecN {
            e: e,
        }
    }

    // Dot product
    pub fn dot(&self, v: VecN<T, N>) -> T where T: Mul<Output = T> + Sum + Copy {
        (self.e).iter()
                .zip((v.e).iter())
                .map(|(&x, &y)| x * y)
                .sum()
    }

    // Left contraction
    // pub fn left_contract(&self, v: &BiVecN) -> VecN {
    //     -(&v.m * self)
    // }

    // Normalize
    pub fn normalize(&self) -> VecN<T, N> where T: Mul<Output = T> + Div<Output = T> + Sqrt + Sum + Copy {
        let mag: T = (self.e).iter()
                                .map(|&x| x*x)
                                .sum::<T>().sqrt();
        VecN::new(std::array::from_fn(|i| self.e[i] / mag))
    }

    // Length
    pub fn length(&self) -> T where T: Mul<Output = T> + Sqrt + Sum + Copy {
        (self.e).iter()
                .map(|&x| x*x)
                .sum::<T>().sqrt()
    }

    // Length squared
    pub fn length_sqr(&self) -> T where T: Mul<Output = T> + Sum + Copy {
        (self.e).iter()
                .map(|&x| x*x)
                .sum::<T>()
    }

    // pub fn orthonormal_basis(&self) -> Vec<VecN> {
    //     let dim = self.e.len();
    //
    //     let normal = self.normalize();
    //
    //     let mut vecs: Vec<VecN> = Vec::new();
    //     let mut maxdot: f64 = f64(0.0);
    //     let mut maxi: usize = 0;
    //
    //     for i in 0..dim {
    //         let v = VecN::basis(dim, i);
    //         let d = v.dot(&normal);
    //         vecs.push(v - &normal * d);
    //         if d.abs() > maxdot {
    //             maxdot = d.abs();
    //             maxi = i;
    //         }
    //     }
    //     vecs.remove(maxi);
    //
    //     for j in 0..(dim-1) {
    //         let vec = vecs[j].normalize();
    //         for k in j+1..(dim-1) {
    //             vecs[k] = &vecs[k] - (vec.dot(&vecs[k]) * &vec);
    //         }
    //     }
    //
    //     vecs
    // }

    pub fn rotate(&self, i: usize, j: usize, angle: T) -> VecN<T, N> where T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + CosSin + Copy {
        let mut r = self.clone();

        let a = r.e[i];
        let b = r.e[j];

        r.e[i] = a * angle.cos() + b * angle.sin();
        r.e[j] = b * angle.cos() - a * angle.sin();

        r
    }

    // Zero
    pub fn zero() -> Self where T: Zero + Copy {
        Self::new([T::zero(); N])
    }

    pub fn default() -> Self where T: Default + Copy {
        Self::new([T::default(); N])
    }

    // Basis element
    pub fn basis(element: usize) -> Self where T: Zero + One + Copy {
        let mut vec = Self::zero();
        vec.e[element] = T::one();
        vec
    }
}