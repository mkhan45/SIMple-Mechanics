use ggez;
use ggez::graphics::{self, DrawMode, MeshBuilder};

use specs::prelude::*;

use crate::resources::{CreateShapeCentered, CreationData, MousePos, MouseStartPos, ShapeInfo};

pub struct DrawCreationGUISys<'m> {
    pub mesh_builder: &'m mut MeshBuilder,
}

impl<'a, 'm> System<'a> for DrawCreationGUISys<'m> {
    type SystemData = (
        Read<'a, CreationData>,
        Read<'a, CreateShapeCentered>,
        Read<'a, MouseStartPos>,
        Read<'a, MousePos>,
    );

    fn run(
        &mut self,
        (create_shape_data, create_shape_centered, mouse_start_pos, mouse_pos): Self::SystemData,
    ) {
        if let (Some(create_shape_data), Some(start_pos)) =
            (create_shape_data.0.as_ref(), mouse_start_pos.0)
        {
            let mouse_drag_vec = mouse_pos.0 - start_pos;
            match (create_shape_data, create_shape_centered.0) {
                (ShapeInfo::Rectangle(_), true) => {
                    let v = mouse_drag_vec.abs();
                    self.mesh_builder.rectangle(
                        graphics::DrawMode::stroke(0.1),
                        graphics::Rect::new(
                            start_pos.x - v.x,
                            start_pos.y - v.y,
                            v.x * 2.0,
                            v.y * 2.0,
                        ),
                        graphics::WHITE,
                    );
                }
                (ShapeInfo::Rectangle(_), false) => {
                    let (start_pos, extents) = if mouse_drag_vec.y > 0.0 {
                        (start_pos, mouse_drag_vec)
                    } else {
                        (start_pos + mouse_drag_vec, -mouse_drag_vec)
                    };

                    self.mesh_builder.rectangle(
                        graphics::DrawMode::stroke(0.1),
                        graphics::Rect::new(start_pos.x, start_pos.y, extents.x, extents.y),
                        graphics::WHITE,
                    );
                }
                (ShapeInfo::Circle(_), true) => {
                    let r = mouse_drag_vec.magnitude();
                    self.mesh_builder.circle(
                        DrawMode::stroke(0.1),
                        [start_pos.x, start_pos.y],
                        r,
                        0.01,
                        graphics::WHITE,
                    );
                }
                (ShapeInfo::Circle(_), false) => {
                    let r = mouse_drag_vec.magnitude() / 2.0;
                    self.mesh_builder.circle(
                        DrawMode::stroke(0.1),
                        [
                            start_pos.x + mouse_drag_vec.x / 2.0,
                            start_pos.y + mouse_drag_vec.y / 2.0,
                        ],
                        r,
                        0.01,
                        graphics::WHITE,
                    );
                }
                _ => {}
            }
        }

        if let Some(ShapeInfo::Polygon(Some(points))) = &create_shape_data.0 {
            let _ = self.mesh_builder.line(
                points
                    .iter()
                    .map(|p| [p.x, p.y])
                    .collect::<Vec<[f32; 2]>>()
                    .as_slice(),
                0.1,
                graphics::WHITE,
            );
        }
    }
}
