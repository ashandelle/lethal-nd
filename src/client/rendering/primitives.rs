use core::f64;

use mathnd::vecn::VecN;

use crate::Vecf64;

// Not efficient, // TODO: https://tavianator.com/2022/ray_box_boundary.html
pub fn rayintersect_aabb<const N: usize>(pos: Vecf64<N>, dir: Vecf64<N>, min: Vecf64<N>, max: Vecf64<N>) -> Option<f64> {
    let (mut tmin, mut tmax) = (0.0, f64::INFINITY);

    let dir_inv: [f64; N] = std::array::from_fn(|i| 1.0 / dir.e[i]);

    for d in 0..N {
        let t1 = (min.e[d] - pos.e[d]) * dir_inv[d];
        let t2 = (max.e[d] - pos.e[d]) * dir_inv[d];

        tmin = t1.max(tmin).min(t2.max(tmin));
        tmax = t1.min(tmax).max(t2.min(tmax));
    }

    match tmin <= tmax {
        true => Some(tmin),
        false => None,
    }
}

pub fn aabb_normal<const N: usize>(pos: Vecf64<N>, min: Vecf64<N>, max: Vecf64<N>) -> Vecf64<N> {
    let c = (min + max) * 0.5;
    let d = (max - min) * 0.5;
    let p = pos - c;

    VecN::new(std::array::from_fn(|i| {
        ((1.0 + 1e-8) * p.e[i] / d.e[i]).trunc()
    })).normalized()
}