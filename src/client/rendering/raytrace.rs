use mathnd::vecn::VecN;

use crate::{Vecf64, client::{Object, RenderObject, aabb_normal, rayintersect_aabb}, world::world::World};

pub fn raytrace<const N: usize>(renderobjects: &Vec<RenderObject<N>>, pos: Vecf64<N>, dir: Vecf64<N>) -> Option<(usize, f64, Vecf64<N>)> where [(); N - 1]: Sized {
    let mut min_dist = f64::INFINITY;
    let mut min_id = 0;
    
    for (id, object) in renderobjects.iter().enumerate() {
        if let Some(dist) = match object.object {
            Object::AABB { min, max } => {
                rayintersect_aabb(pos, dir, min, max)
            },
        } {
            if dist < min_dist {
                min_dist = dist;
                min_id = id;
            }
        }
    }

    if min_dist.is_infinite() {
        return None;
    }

    let pos = pos + dir * min_dist;

    if let Some(object) = renderobjects.get(min_id) {
        let normal = match object.object {
            Object::AABB { min, max } => {
                aabb_normal(pos, min, max)
            },
        };

        Some((min_id, min_dist, normal))
    } else {
        None
    }
}