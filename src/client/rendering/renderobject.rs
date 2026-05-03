use crate::Vecf64;

pub enum RenderObject<const N: usize> {
    // Sphere,
    AABB {
        min: Vecf64<N>,
        max: Vecf64<N>,
    },
}