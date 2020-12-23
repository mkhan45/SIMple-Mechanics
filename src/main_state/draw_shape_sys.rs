use ggez::graphics::{self, MeshBuilder};

use specs::prelude::*;

use crate::components::{Collider, Color};
use crate::resources::Selected;
use crate::ColliderSet;

use super::util::{draw_circle, draw_rect};

use ncollide2d as nc;

pub struct DrawShapesSys<'m> {
    pub mesh_builder: &'m mut MeshBuilder,
}

impl<'a, 'm> System<'a> for DrawShapesSys<'m> {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Collider>,
        ReadStorage<'a, Color>,
        ReadExpect<'a, ColliderSet>,
        Read<'a, Selected>,
    );

    fn run(&mut self, (entities, colliders, colors, collider_set, selected): Self::SystemData) {
        (&colliders, &colors, &entities)
            .join()
            .for_each(|(collider_comp, color, e)| {
                let collider = collider_set
                    .get(collider_comp.coll_handle)
                    .expect("error getting collider to draw");

                let (pos, rot) = {
                    let isometry = collider.position();
                    let na_vector = isometry.translation.vector;
                    ([na_vector.x, na_vector.y], isometry.rotation.angle())
                };

                if collider.shape().is_shape::<nc::shape::Ball<f32>>() {
                    let shape = collider
                        .shape()
                        .downcast_ref::<nc::shape::Ball<f32>>()
                        .expect("bad shape");

                    draw_circle(
                        &mut self.mesh_builder,
                        pos,
                        rot,
                        shape.radius(),
                        color.0,
                        false,
                    );

                    if let Some(selected) = selected.0 {
                        if selected == e {
                            draw_circle(
                                &mut self.mesh_builder,
                                pos,
                                rot,
                                shape.radius(),
                                graphics::Color::new(1.0, 0.0, 0.0, 1.0),
                                true,
                            );
                        }
                    }
                } else if collider.shape().is_shape::<nc::shape::Cuboid<f32>>() {
                    let shape = collider
                        .shape()
                        .downcast_ref::<nc::shape::Cuboid<f32>>()
                        .expect("bad shape");

                    draw_rect(
                        &mut self.mesh_builder,
                        pos,
                        rot,
                        *shape.half_extents(),
                        color.0,
                        false,
                    );

                    if let Some(selected) = selected.0 {
                        if selected == e {
                            draw_rect(
                                &mut self.mesh_builder,
                                pos,
                                rot,
                                *shape.half_extents(),
                                graphics::Color::new(1.0, 0.0, 0.0, 1.0),
                                true,
                            );
                        }
                    }
                }
            });
    }
}
