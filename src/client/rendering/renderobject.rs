use crate::Vecf64;

pub struct RenderObject<const N: usize> {
    pub object: Object<N>,
    pub color: (f64, f64, f64),
}

pub enum Object<const N: usize> {
    // Sphere,
    AABB {
        min: Vecf64<N>,
        max: Vecf64<N>,
    },
}