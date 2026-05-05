use macroquad::{color::Color, texture::Image};
use mathnd::vecn::VecN;
use rayon::prelude::*;

use crate::{Vecf64, client::{ClientSettings, Object, RenderObject, raytrace}, util::map, world::world::{ClientServer, World}};

pub fn render<const N: usize>(settings: &ClientSettings<N>, world: &World<N>, screen_image: &mut Image) where [(); N - 1]: Sized {
    let (width, height) = (screen_image.width as usize, screen_image.height as usize);
    let ratio = height as f64 / width as f64;

    if let ClientServer::Client {
        id,
        ..
    } = world.clientserver {
        if let Some(player) = world.entities.get(&id) {
            let pos = player.position;
            // let rot = player.rotation;

            let mut renderobjects: Vec<RenderObject<N>> = Vec::new();

            for (eid, entity) in &world.entities {
                if id != *eid {
                    renderobjects.push(RenderObject {
                        object: Object::AABB {
                            min: entity.position - VecN::new([0.5; N]),
                            max: entity.position + VecN::new([0.5; N])
                        },
                        color: (1.0, 1.0, 1.0),
                    });
                }
            }

            // renderobjects.push(RenderObject {
            //     object: Object::AABB {
            //         min: VecN::basis(1)*2.0 + VecN::new([-0.5; N]),
            //         max: VecN::basis(1)*2.0 + VecN::new([0.5; N])
            //     },
            //     color: (1.0, 0.0, 0.0),
            // });

            // renderobjects.push(RenderObject {
            //     object: Object::AABB {
            //         min: -VecN::basis(1)*2.0 + VecN::new([-0.5; N]),
            //         max: -VecN::basis(1)*2.0 + VecN::new([0.5; N])
            //     },
            //     color: (0.0, 0.5, 0.5),
            // });

            // renderobjects.push(RenderObject {
            //     object: Object::AABB {
            //         min: VecN::basis(2)*2.0 + VecN::new([-0.5; N]),
            //         max: VecN::basis(2)*2.0 + VecN::new([0.5; N])
            //     },
            //     color: (0.0, 1.0, 0.0),
            // });

            // renderobjects.push(RenderObject {
            //     object: Object::AABB {
            //         min: -VecN::basis(2)*2.0 + VecN::new([-0.5; N]),
            //         max: -VecN::basis(2)*2.0 + VecN::new([0.5; N])
            //     },
            //     color: (0.5, 0.0, 0.5),
            // });

            // renderobjects.push(RenderObject {
            //     object: Object::AABB {
            //         min: VecN::basis(3)*2.0 + VecN::new([-0.5; N]),
            //         max: VecN::basis(3)*2.0 + VecN::new([0.5; N])
            //     },
            //     color: (0.0, 0.0, 1.0),
            // });

            // renderobjects.push(RenderObject {
            //     object: Object::AABB {
            //         min: -VecN::basis(3)*2.0 + VecN::new([-0.5; N]),
            //         max: -VecN::basis(3)*2.0 + VecN::new([0.5; N])
            //     },
            //     color: (0.5, 0.5, 0.0),
            // });

            // screen_image.get_image_data_mut().par_chunks_mut(width).enumerate().for_each(|(y, strip)| {
            //     for (x, pixel) in strip.iter_mut().enumerate() {
            screen_image.get_image_data_mut().par_iter_mut().enumerate().for_each(|(i, pixel)| {
                let (x, y) = (i % width, i / width);

                let x = map(x as f64, 0.0, (width-1) as f64, -1.0, 1.0);
                let y = map(y as f64, 0.0, (height-1) as f64, ratio, -ratio);

                let mut dir: Vecf64<N> = VecN::zero();
                dir.e[0] = y;
                dir.e[1] = 1.0;
                dir.e[2] = x;
                dir.normalize();
                dir = player.camera_to_world_norm(dir);

                let (index, dist, normal) = match raytrace(&renderobjects, pos, dir) {
                    Some((index, dist, normal)) => (Some(index), dist, Some(normal)),
                    None => (None, f64::INFINITY, None),
                };

                let v = 1.0 / (1.0 + dist / 10.0);

                let mut color = (1.0, 1.0, 1.0);

                if let (Some(index), Some(normal)) = (index, normal) {
                    color = renderobjects.get(index).unwrap().color;
                }

                *pixel = Color::new((color.0 * v) as f32, (color.1 * v) as f32, (color.2 * v) as f32, 1.0).into();
            });
        }
    } else {
        panic!();
    }
}