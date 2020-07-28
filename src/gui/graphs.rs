use ggez::graphics::MeshBuilder;
use specs::storage::BTreeStorage;
use specs::Component;

use std::collections::VecDeque;

use crate::main_state::MainState;

// use csv;

pub trait Graph {
    fn draw(&self, builder: &mut MeshBuilder);
    fn serialize_csv(&self);
}

pub trait LineGraph {
    fn add_val(&mut self, val: f32);
    fn points(&self) -> &[[f32; 2]];
    fn name(&self) -> String;
    fn shown(&self) -> bool;
    fn max_len(&self) -> usize;
}

impl Graph for dyn LineGraph {
    fn draw(&self, builder: &mut MeshBuilder) {
        use std::f32::{INFINITY, NEG_INFINITY};

        if self.points().len() < 3 {
            return;
        }

        let (min, max) = self
            .points()
            .iter()
            .fold((INFINITY, NEG_INFINITY), |(min, max), [_, v]| {
                (min.min(*v), max.max(*v))
            });

        let midpoint = (min + max) / 2.0;
        let scale_fac = 8.0 / (max - min).max(1.0e-10);

        builder
            .line(
                dbg!(self
                    .points()
                    .iter()
                    .map(|[x, v]| [*x, 5.5 - (v - midpoint) * scale_fac])
                    .collect::<Vec<[f32; 2]>>()
                    .as_slice()),
                0.1,
                ggez::graphics::Color::new(1.0, 0.0, 0.0, 1.0),
            )
            .unwrap();
    }

    fn serialize_csv(&self) {
        let mut writer = csv::Writer::from_writer(std::io::stdout());

        writer.write_record(&[self.name()]).unwrap();
        self.points().iter().for_each(|[_, val]| {
            writer.write_record(&[val.to_string()]).unwrap();
        });
        writer.flush().unwrap();
    }
}

macro_rules! create_linegraph {
    ($structname:ident, $name:expr) => {
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
            fn points(&self) -> &[[f32; 2]] {
                &self.data.as_slices().0
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
                self.data.make_contiguous();
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
        }
    };
}

impl<'a, 'b> MainState<'a, 'b> {
    pub fn draw_graphs(&self, builder: &mut MeshBuilder) {
        use ggez::graphics::{DrawMode, Rect, BLACK, WHITE};
        use specs::prelude::*;

        let speed_graphs = self.world.read_storage::<SpeedGraph>();

        let mut first_iter = true;
        speed_graphs.join().for_each(|graph| {
            if graph.shown {
                if first_iter {
                    first_iter = false;
                    builder.rectangle(
                        DrawMode::stroke(0.1),
                        Rect::new(0.0, 0.0, 10.0, 10.0),
                        WHITE,
                    );
                    builder.rectangle(DrawMode::fill(), Rect::new(0.0, 0.0, 10.0, 10.0), BLACK);
                }
                Graph::draw(graph as &dyn LineGraph, builder);
            }
        });
    }
}

create_linegraph!(SpeedGraph, "Speed");
create_linegraph!(RotVelGraph, "Rotational Velocity");
create_linegraph!(XPosGraph, "X Position");
create_linegraph!(YPosGraph, "Y Position");
