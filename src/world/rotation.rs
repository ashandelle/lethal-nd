use std::{fmt::Display, iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, Sub}};

use mathnd::{bivecn::BiVecN, matn::MatN, traits::{CosSin, Sqrt}, vecn::VecN};
use num_traits::{FromPrimitive, One, Signed, Zero};

use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Rotation<T, const N: usize> where [(); N - 1]: Sized {
    pub horizontal: MatN<T, {N - 1}>,
    pub vertical: T,
}

// #[derive(Encode, Decode, Debug)]
// pub struct RotationQuantized<const N: usize> where [(); N - 1]: Sized {
//     pub horizontal: MatNQuantized<u16, {N - 1}>,
//     pub vertical: u8,
// }

impl<T, const N: usize> Rotation<T, N> where
    T: Display + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign + DivAssign + PartialOrd + Sum + CosSin + Sqrt + Signed + Zero + One + FromPrimitive + Copy,
    [(); N - 1 ]: Sized {
    pub fn rotate_by_bivector(&self, bivec: BiVecN<T, N>, eps: T) -> Rotation<T, N> {
        let vert = bivec.get_ij(0, 1);

        let bivec: MatN<T, {N-1}> = MatN::new(
            std::array::from_fn(|i| VecN::new(
                std::array::from_fn(|j|
                    bivec.m.e[i+1].e[j+1]
                )
            ))
        );
        
        let dr: MatN<T, {N-1}> = MatN::identity() + bivec;

        let vertical = self.vertical + vert;
        let horizontal = (dr * self.horizontal).orthonormalized(eps, 128);

        Rotation { horizontal, vertical }
    }

    pub fn rotate_horizontal(&self, vec: VecN<T, N>) -> VecN<T, N> {
        let mut arr = [T::zero(); N - 1];
        arr.copy_from_slice(&vec.e[1..]);
        let mut v = VecN { e: arr };
        v = self.horizontal * v;
        let mut arr = [T::zero(); N];
        arr[1..].copy_from_slice(&v.e);
        arr[0] = vec.e[0];
        VecN { e: arr }
    }

    pub fn transpose_rotate_horizontal(&self, vec: VecN<T, N>) -> VecN<T, N> {
        let mut arr = [T::zero(); N - 1];
        arr.copy_from_slice(&vec.e[1..]);
        let mut v = VecN { e: arr };
        v = self.horizontal.transposed() * v;
        let mut arr = [T::zero(); N];
        arr[1..].copy_from_slice(&v.e);
        arr[0] = vec.e[0];
        VecN { e: arr }
    }

    pub fn rotate(&self, vec: VecN<T, N>) -> VecN<T, N> {
        self.rotate_horizontal(vec).rotate(0, 1, self.vertical)
    }

    pub fn transpose_rotate(&self, vec: VecN<T, N>) -> VecN<T, N> {
        self.transpose_rotate_horizontal(vec.rotate(1, 0, self.vertical))
    }

    pub fn identity() -> Self {
        Rotation {
            horizontal: MatN::identity(),
            vertical: T::zero()
        }
    }
}