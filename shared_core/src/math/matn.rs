use std::{iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub}};

use crate::math::{Abs, FromUsize, One, Sqrt, Two, VecN, Zero};

#[derive(Debug, Clone, Copy)]
pub struct MatN<T, const N: usize> {
    pub(crate) e: [VecN<T, N>; N]
}

impl<T, const N: usize> PartialEq for MatN<T, N> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.e == other.e
    }
}

impl<T, const N: usize> Neg for MatN<T, N> where T: Neg<Output = T> + Copy {
    type Output = MatN<T, N>;
    fn neg(self) -> MatN<T, N> {
        MatN::new(std::array::from_fn(|i| -self.e[i]))
    }
}

impl<T, const N: usize> Add for MatN<T, N> where T: Add<Output = T> + Copy {
    type Output = MatN<T, N>;
    fn add(self, v: MatN<T, N>) -> MatN<T, N> {
        MatN::new(std::array::from_fn(|i| self.e[i] + v.e[i]))
    }
}

impl<T, const N: usize> Sub for MatN<T, N> where T: Sub<Output = T> + Copy {
    type Output = MatN<T, N>;
    fn sub(self, v: MatN<T, N>) -> MatN<T, N> {
        MatN::new(std::array::from_fn(|i| self.e[i] - v.e[i]))
    }
}

impl<T, const N: usize> Mul<T> for MatN<T, N> where T: Mul<Output = T> + Copy {
    type Output = MatN<T, N>;
    fn mul(self, s: T) -> MatN<T, N> {
        MatN::new(std::array::from_fn(|i| self.e[i] * s))
    }
}

impl<T, const N: usize> Div<T> for MatN<T, N> where T: Div<Output = T> + Copy {
    type Output = MatN<T, N>;
    fn div(self, s: T) -> MatN<T, N> {
        MatN::new(std::array::from_fn(|i| self.e[i] / s))
    }
}

impl<T, const N: usize> Mul<VecN<T, N>> for MatN<T, N> where T: Mul<Output = T> + Sum + Copy {
    type Output = VecN<T, N>;
    fn mul(self, v: VecN<T, N>) -> VecN<T, N> {
        VecN::new(std::array::from_fn(|i| self.e[i].dot(v)))
    }
}

impl<T, const N: usize> Mul for MatN<T, N> where T: Mul<Output = T> + Sum + Copy {
    type Output = MatN<T, N>;
    fn mul(self, v: MatN<T, N>) -> MatN<T, N> {
        let t = v.transpose();
        MatN::new(std::array::from_fn(|i| t * self.e[i]))
    }
}

// impl fmt::Display for MatN {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[");
//         for i in 0..self.e.len() {
//             write!(f, "{}", self.e[i]);
//             if i != self.e.len()-1 {
//                 write!(f, ", ");
//             }
//         }
//         write!(f, "]")
//     }
// }

impl<T, const N: usize> MatN<T, N> {
    pub fn new(e: [VecN<T, N>; N]) -> Self {
        MatN {
            e: e,
        }
    }

    // Dot product
    pub fn dot(&self, m: &MatN<T, N>) -> T where T: Mul<Output = T> + Sum + Copy {
        (self.e).iter()
                .zip((m.e).iter())
                .map(|(&x, &y)| x.dot(y))
                .sum::<T>()
    }

    // To BiVecN
    // pub fn to_bivecn(self) -> BiVecN {
    //     BiVecN {
    //         m: self,
    //     }.skew()
    // }

    // Normalize basis vectors
    pub fn normalize_basis(&self) -> MatN<T, N> where T: Mul<Output = T> + Div<Output = T> + Sum + Sqrt + Copy {
        MatN::new(std::array::from_fn(|i| self.e[i].normalize()))
    }

    // Orthonormalize
    pub fn orthonormalize(&self, eps: T, max: usize) -> MatN<T, N> where
        T: Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign + DivAssign + PartialOrd + Sum + Sqrt + Abs + Zero + FromUsize + Copy {
        let mut mat = self.normalize_basis();
        let n = (N - 1).max(2);

        let mut dot: T = T::zero();
        let mut iter = 0;

        while iter == 0 || (dot > eps && iter < max) {
            let mut tmp = mat.clone();

            dot = T::zero();
            for i in 0..N {
                for j in (i+1)..N {
                    let d = mat.e[i].dot(mat.e[j]);
                    dot += d.abs();
                    tmp.e[i] = tmp.e[i] - (mat.e[j] * d / T::fromusize(n));
                    tmp.e[j] = tmp.e[j] - (mat.e[i] * d / T::fromusize(n));
                }
            }
            dot /= T::fromusize((N*N - N) / 2);

            mat = tmp.normalize_basis();
            iter += 1;
        }

        mat
    }

    // Length
    pub fn length(&self) -> T where T: Mul<Output = T> + Sqrt + Sum + Copy {
        (self.e).iter()
                .map(|x| x.length_sqr())
                .sum::<T>().sqrt()
    }

    // Length squared
    pub fn length_sqr(&self) -> T where T: Mul<Output = T> + Sum + Copy {
        (self.e).iter()
                .map(|x| x.length_sqr())
                .sum::<T>()
    }

    // Transpose
    pub fn transpose(&self) -> MatN<T, N> where T: Copy {
        let mut mat: MatN<T, N> = self.clone();
        for i in 0..N {
            for j in 0..N {
                mat.e[j].e[i] = self.e[i].e[j];
            }
        }
        mat
    }

    // pub fn mult_transpose(&self, v: VecN) -> VecN {
    //     VecN {
    //         e: self.transpose().e.iter()
    //             .map(|x| x.dot(&v))
    //             .collect(),
    //     }
    // }

    // pub fn mult_transpose_bivecn(&self, v: &BiVecN) -> BiVecN {
    //     BiVecN {
    //         m: (self.transpose() * &v.m) * self,
    //     }.skew()
    // }

    // Inverse
    pub fn inverse(self, eps: T, max: usize) -> MatN<T, N> where
        T: Mul<Output = T> + Sub<Output = T> + Div<Output = T> + PartialOrd + Sum + Zero + One + Two + FromUsize + Copy {
        let mut inv = MatN::identity();

        let mut iter = 0;
        let mut len = T::zero();
        let I = MatN::identity();

        while iter == 0 || (len > eps && iter < max) {
            let mut id = inv * self;
            inv = (inv * T::two()) - (id * inv);
            id = id - I;
            len = id.dot(&id) / T::fromusize(N*N);
            iter+=1;
        }

        inv
    }

    // Matrix rotating v1 to v2
    pub fn from_vecn(v1: VecN<T, N>, v2: VecN<T, N>) -> Self where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Sqrt + Sum + Zero + One + Two + Copy {
        let n1 = v1.normalize();
        let n2 = v2.normalize();
        let v3 = n1 + n2;
        (Self::identity() -
        Self::mult_transpose_vecn(v3, v3) / (T::one() + n1.dot(n2)) +
        Self::mult_transpose_vecn(n2, n1) * T::two()) *
        (v2.length() / v1.length())
    }

    // Matrix rotating v1 to v2
    // pub fn from_vecn_interpolate(v1: &VecN, v2: &VecN, t: N64) -> Self {
    //     let v3 = v1 + v2;
    //
    //     let mut cos = v1.dot(&v2).clamp(-1.0, 1.0);
    //     let mut sin = (1.0 - cos*cos).sqrt();
    //     let theta = cos.acos();
    //
    //     if theta.abs() < 1e-8 {
    //         return MatN::identity();
    //     }
    //
    //     let mut c = -Self::mult_transpose_vecn(&v3, &v3) / (1.0 + cos);
    //     let mut s = 2.0 * Self::mult_transpose_vecn(&v2, &v1);
    //
    //     c = c / (1.0 - cos);
    //     s = s / sin;
    //
    //     cos = (theta * t).cos();
    //     sin = (theta * t).sin();
    //
    //     c = c * (1.0 - cos);
    //     s = s * sin;
    //
    //     MatN::identity() + c + s
    // }

    // Matrix formed by v1 * v2^T
    pub fn mult_transpose_vecn(v1: VecN<T, N>, v2: VecN<T, N>) -> Self where T: Mul<Output = T> + Copy {
        Self::new(std::array::from_fn(|i| v2 * v1.e[i]))
    }

    // Zero
    pub fn zero() -> Self where T: Zero + Copy {
        Self::new([VecN::zero(); N])
    }

    // Identity
    pub fn identity() -> Self where T: Zero + One + Copy {
        let mut mat = Self::zero();
        for i in 0..N {
            mat.e[i].e[i] = T::one();
        }
        mat
    }
}