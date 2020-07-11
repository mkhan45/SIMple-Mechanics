use smallvec::SmallVec;

use ggez::graphics;

use specs::prelude::*;

use crate::{ColliderSet, GeometricalWorld, Point, Vector};

use crate::resources;
use ncollide2d as nc;

pub fn draw_circle(
    mesh_builder: &mut ggez::graphics::MeshBuilder,
    pos: [f32; 2],
    rot: f32,
    rad: f32,
    color: graphics::Color,
    outline: bool,
) {
    let drawmode = if outline {
        graphics::DrawMode::stroke(rad * 0.05)
    } else {
        graphics::DrawMode::fill()
    };

    mesh_builder.circle(drawmode, pos, rad, 0.01, color);

    mesh_builder.circle(
        drawmode,
        [
            pos[0] + rad * rot.cos() * 0.75,
            pos[1] + rad * rot.sin() * 0.75,
        ],
        rad * 0.15,
        0.01,
        graphics::BLACK,
    );
}

pub fn draw_rect(
    mesh_builder: &mut ggez::graphics::MeshBuilder,
    center_pos: [f32; 2],
    rot: f32,
    half_extents: Vector,
    color: graphics::Color,
    outline: bool,
) {
    let rot_cos = rot.cos();
    let rot_sin = rot.sin();

    // rect points in clockwise (important for ggez)
    let _points = [
        Point::new(
            center_pos[0] - half_extents.x,
            center_pos[1] - half_extents.y,
        ),
        Point::new(
            center_pos[0] + half_extents.x,
            center_pos[1] - half_extents.y,
        ),
        Point::new(
            center_pos[0] + half_extents.x,
            center_pos[1] + half_extents.y,
        ),
        Point::new(
            center_pos[0] - half_extents.x,
            center_pos[1] + half_extents.y,
        ),
    ]
    .iter()
    .map(|point| {
        // new x position is cos(theta) * (p.x - c.x) - sin(theta) * (p.y - c.y) + c.x
        // new y position is sin(theta) * (p.x - c.x) + cos(theta) * (p.y - c.y) + c.y
        [
            rot_cos * (point.x - center_pos[0]) - rot_sin * (point.y - center_pos[1])
                + center_pos[0],
            rot_sin * (point.x - center_pos[0])
                + rot_cos * (point.y - center_pos[1])
                + center_pos[1],
        ]
    })
    .collect::<SmallVec<[[f32; 2]; 4]>>();

    let points = _points.as_slice();

    let drawmode = if outline {
        graphics::DrawMode::stroke(half_extents.x.min(half_extents.y) * 0.125)
    } else {
        graphics::DrawMode::fill()
    };

    mesh_builder
        .polygon(drawmode, points, color)
        .expect("error drawing rotated rect");
}

pub fn get_hovered_shape(world: &World) -> Option<Entity> {
    let geometrical_world = world.fetch::<GeometricalWorld>();
    let colliders = world.fetch::<ColliderSet>();
    let mouse_point = world.fetch::<resources::MousePos>().0;

    geometrical_world
        .interferences_with_point(
            &*colliders,
            &Point::new(mouse_point.x, mouse_point.y),
            &nc::pipeline::CollisionGroups::new(),
        )
        .map(|obj| {
            let specs_hand = obj.1.user_data().unwrap();
            *specs_hand.downcast_ref::<Entity>().unwrap()
        })
        .next()
}
