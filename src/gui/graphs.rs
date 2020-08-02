use ggez::graphics::{self, Text};
use specs::storage::BTreeStorage;
use specs::Component;

use std::collections::VecDeque;

use crate::{
    components,
    main_state::MainState,
    resources::{GraphMinMax, GraphPosData},
    RigidBody,
};
use graphics::{MeshBuilder, Rect, Scale, TextFragment};

// use csv;

pub trait Graph {
    fn draw(&self, builder: &mut MeshBuilder, color: graphics::Color, min_max: Option<(f32, f32)>);
}

pub trait LineGraph {
    fn add_val(&mut self, val: f32);
    fn points(&self) -> (&[[f32; 2]], &[[f32; 2]]);
    fn name(&self) -> String;
    fn shown(&self) -> bool;
    fn max_len(&self) -> usize;
    fn access_field(rigid_body: &RigidBody) -> f32
    where
        Self: Sized;
}

impl Graph for dyn LineGraph {
    fn draw(
        &self,
        builder: &mut MeshBuilder,
        color: graphics::Color,
        midpoint_scale: Option<(f32, f32)>,
    ) {
        use std::f32::{INFINITY, NEG_INFINITY};

        let (s0, s1) = self.points();
        let (midpoint, scale_fac) = midpoint_scale.unwrap_or_else(|| {
            let (min, max) = s0
                .iter()
                .chain(s1.iter())
                .fold((INFINITY, NEG_INFINITY), |(min, max), [_, v]| {
                    (min.min(*v), max.max(*v))
                });
            let midpoint = (min + max) / 2.0;
            let scale_fac = 8.0 / (max - min).max(1.0 / 8.0);
            (midpoint, scale_fac)
        });

        if s0.len() + s1.len() >= 3 {
            builder
                .line(
                    s0.iter()
                        .chain(s1.iter())
                        .map(|[x, v]| [*x, 5.5 - (v - midpoint) * scale_fac])
                        .collect::<Vec<[f32; 2]>>()
                        .as_slice(),
                    0.1,
                    color,
                )
                .unwrap();
        }
    }
}

macro_rules! create_linegraph {
    ($structname:ident, $name:expr, $access_closure:expr) => {
        #[derive(Debug, Clone, Component)]
        #[storage(BTreeStorage)]
        pub struct $structname {
            pub data: VecDeque<[f32; 2]>,
            pub shown: bool,
            pub max_len: usize,
        }

        impl Default for $structname {
            fn default() -> Self {
                $structname {
                    data: VecDeque::with_capacity(60 * 10 / 4),
                    shown: true,
                    max_len: 60 * 5,
                }
            }
        }

        impl LineGraph for $structname {
            fn points(&self) -> (&[[f32; 2]], &[[f32; 2]]) {
                self.data.as_slices()
            }

            fn add_val(&mut self, val: f32) {
                let num_vals = self.data.len() + 1;
                let step_incr = 10.0 / num_vals as f32;

                self.data.iter_mut().enumerate().for_each(|(i, [x, _])| {
                    *x = step_incr * i as f32;
                });
                if num_vals < self.max_len {
                    self.data.push_back([10.0, val]);
                } else {
                    self.data.pop_front();
                    self.data.push_back([10.0, val]);
                }
            }

            fn name(&self) -> String {
                $name.to_string()
            }

            fn shown(&self) -> bool {
                self.shown
            }

            fn max_len(&self) -> usize {
                self.max_len
            }

            fn access_field(rigid_body: &RigidBody) -> f32 {
                $access_closure(rigid_body)
            }
        }
    };
}

create_linegraph!(SpeedGraph, "Speed", |rigid_body: &RigidBody| rigid_body
    .velocity()
    .linear
    .norm());
create_linegraph!(
    RotVelGraph,
    "Rotational Velocity",
    |rigid_body: &RigidBody| rigid_body.velocity().angular
);
create_linegraph!(XPosGraph, "X Position", |rigid_body: &RigidBody| rigid_body
    .position()
    .translation
    .x);
create_linegraph!(YPosGraph, "Y Position", |rigid_body: &RigidBody| rigid_body
    .position()
    .translation
    .y);
create_linegraph!(XVelGraph, "X Velocity", |rigid_body: &RigidBody| rigid_body
    .velocity()
    .linear
    .x);
create_linegraph!(YVelGraph, "Y Velocity", |rigid_body: &RigidBody| rigid_body
    .velocity()
    .linear
    .y);

impl<'a, 'b> MainState<'a, 'b> {
    pub fn draw_graphs(&self) -> ([Text; 3], MeshBuilder) {
        use specs::prelude::*;

        let mut builder = MeshBuilder::new();

        // let speed_graphs = self.world.read_storage::<SpeedGraph>();
        let colors = self.world.read_storage::<components::Color>();
        let GraphMinMax(min, max) = *self.world.fetch::<GraphMinMax>();
        let midpoint = (min + max) / 2.0;
        let scale_fac = 8.0 / (max - min).max(1.0 / 8.0);

        let mut first_iter = true;
        macro_rules! draw_graphtype {
            ( $graphtype:ident ) => {
                let graph_storages = self.world.read_storage::<$graphtype>();
                (&graph_storages, &colors)
                    .join()
                    .for_each(|(graph, color)| {
                        if graph.shown {
                            if first_iter {
                                first_iter = false;
                                draw_graph_frame(&mut builder);
                            }
                            Graph::draw(
                                graph as &dyn LineGraph,
                                &mut builder,
                                color.0,
                                Some((midpoint, scale_fac)),
                            );
                        }
                    });
            };
        }

        draw_graphtype!(SpeedGraph);
        draw_graphtype!(RotVelGraph);
        draw_graphtype!(XVelGraph);
        draw_graphtype!(YVelGraph);
        draw_graphtype!(XPosGraph);
        draw_graphtype!(YPosGraph);

        let max_text = graphics::Text::new(
            TextFragment::new(format!("{0:.3}", max)).scale(Scale::uniform(25.0)),
        );
        let mid_text = graphics::Text::new(
            TextFragment::new(format!("{0:.3}", midpoint)).scale(Scale::uniform(25.0)),
        );
        let min_text = graphics::Text::new(
            TextFragment::new(format!("{0:.3}", min)).scale(Scale::uniform(25.0)),
        );

        ([max_text, mid_text, min_text], builder)
    }

    pub fn graph_grab_rect(&self) -> Rect {
        let graph_rect = self.world.fetch::<GraphPosData>().0;
        let scale_fac = graph_rect.w / 10.0;
        graphics::Rect::new(
            graph_rect.x + (9.5 * scale_fac),
            graph_rect.y + (9.5 * scale_fac),
            0.5 * scale_fac,
            0.5 * scale_fac,
        )
    }
}

fn draw_graph_frame(builder: &mut MeshBuilder) {
    use ggez::graphics::{DrawMode, BLACK, WHITE};
    builder.rectangle(
        DrawMode::stroke(0.1),
        Rect::new(0.0, 0.0, 10.0, 10.0),
        WHITE,
    );
    builder.rectangle(DrawMode::fill(), Rect::new(0.0, 0.0, 10.0, 10.0), BLACK);
    builder.rectangle(
        DrawMode::fill(),
        Rect::new(9.5, 9.5, 0.5, 0.5),
        graphics::Color::new(0.45, 0.6, 0.85, 1.0),
    );
}
