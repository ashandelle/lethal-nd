use std::{iter::Sum, ops::{Add, Mul, Sub}};

use mathnd::{matn::MatN, traits::CosSin, vecn::VecN};
use num_traits::Zero;

pub struct Rotation<T, const N: usize> where [(); N - 1]: Sized {
    pub horizontal: MatN<T, {N - 1}>,
    pub vertical: T,
}

pub struct RotationQuantized<const N: usize> where [(); N - 1]: Sized {
    pub horizontal: MatN<u16, {N - 1}>,
    pub vertical: u8,
}

impl<T, const N: usize> Rotation<T, N> where T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Sum + CosSin + Zero + Copy, [(); N - 1 ]: Sized {
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
        v = self.horizontal.transpose() * v;
        let mut arr = [T::zero(); N];
        arr[1..].copy_from_slice(&v.e);
        arr[0] = vec.e[0];
        VecN { e: arr }
    }

    pub fn rotate(&self, vec: VecN<T, N>) -> VecN<T, N> {
        let mut arr = [T::zero(); N - 1];
        arr.copy_from_slice(&vec.e[1..]);
        let mut v = VecN { e: arr };
        v = self.horizontal * v;
        let mut arr = [T::zero(); N];
        arr[1..].copy_from_slice(&v.e);
        arr[0] = vec.e[0];
        VecN { e: arr }.rotate(0, 1, self.vertical)
    }

    pub fn transpose_rotate(&self, vec: VecN<T, N>) -> VecN<T, N> {
        let mut arr = [T::zero(); N - 1];
        arr.copy_from_slice(&(vec.rotate(1, 0, self.vertical)).e[1..]);
        let mut v = VecN { e: arr };
        v = self.horizontal.transpose() * v;
        let mut arr = [T::zero(); N];
        arr[1..].copy_from_slice(&v.e);
        arr[0] = vec.e[0];
        VecN { e: arr }
    }
}