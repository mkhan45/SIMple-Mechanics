#![allow(clippy::type_complexity)]
use specs::prelude::*;

use crate::{BodySet, MechanicalWorld, RigidBody, Selected};

use crate::components::*;
use crate::gui::graphs::{LineGraph, RotVelGraph, SpeedGraph};

use crate::{resources::*, Vector};

pub struct SelectedMoveSys;

impl<'a> System<'a> for SelectedMoveSys {
    type SystemData = (
        ReadStorage<'a, Selected>,
        ReadStorage<'a, PhysicsBody>,
        Read<'a, MousePos>,
        Option<Read<'a, MechanicalWorld>>,
        Option<Write<'a, BodySet>>,
    );

    fn run(
        &mut self,
        (selected, physics_body, mouse_pos, mechanical_world, mut body_set): Self::SystemData,
    ) {
        (&selected, &physics_body)
            .join()
            .for_each(|(_, physics_body)| {
                let body_set = body_set.as_mut().unwrap();
                let rigid_body = body_set
                    .get_mut(physics_body.body_handle)
                    .unwrap()
                    .downcast_mut::<RigidBody>()
                    .unwrap();

                let pos = rigid_body.position().translation.vector;
                let new_vel = mouse_pos.0 - pos;

                let physics_step = mechanical_world.as_ref().unwrap().timestep();

                rigid_body.set_linear_velocity(new_vel / physics_step);
            });
    }
}

pub struct MinMaxGraphSys;
impl<'a> System<'a> for MinMaxGraphSys {
    type SystemData = (
        ReadStorage<'a, SpeedGraph>,
        ReadStorage<'a, RotVelGraph>,
        Write<'a, GraphMinMax>,
    );

    fn run(&mut self, (speed_graphs, rotvel_graphs, mut min_max): Self::SystemData) {
        let (mut min, mut max) = (std::f32::INFINITY, std::f32::NEG_INFINITY);

        macro_rules! minmax_graph_storage {
            ( $graph_storage:expr ) => {
                $graph_storage.join().for_each(|graph| {
                    let (s0, s1) = graph.points();
                    s0.iter().chain(s1.iter()).for_each(|[_, v]| {
                        min = min.min(*v);
                        max = max.max(*v);
                    });
                });
            };
        }
        minmax_graph_storage!(speed_graphs);
        minmax_graph_storage!(rotvel_graphs);

        min_max.0 = min;
        min_max.1 = max;
    }
}

pub struct GraphTransformSys;
impl<'a> System<'a> for GraphTransformSys {
    type SystemData = (
        Read<'a, MousePos>,
        Write<'a, GraphPosData>,
        Read<'a, MovingGraph>,
        Read<'a, ScalingGraph>,
    );

    fn run(
        &mut self,
        (mouse_pos, mut graph_pos_data, moving_graph, scaling_graph): Self::SystemData,
    ) {
        if moving_graph.0 {
            graph_pos_data.0.x = mouse_pos.0.x - graph_pos_data.0.w;
            graph_pos_data.0.y = mouse_pos.0.y - graph_pos_data.0.h;
        }

        if scaling_graph.0 {
            let scale_mag = {
                let graph_pos_vec = Vector::new(graph_pos_data.0.x, graph_pos_data.0.y);
                (mouse_pos.0 - graph_pos_vec).norm()
            };

            // sqrt(200) is the diagonal length of the drawing area of the graph meshbuilder
            let scale_fac = scale_mag / (200.0f32).sqrt();
            let side_len = scale_fac * 10.0;

            graph_pos_data.0.w = side_len;
            graph_pos_data.0.h = side_len;
        }
    }
}

macro_rules! make_graphsys {
    ( $sys:ident, $graphcomp:ident, $access_fn:expr ) => {
        pub struct $sys;
        impl<'a> System<'a> for $sys {
            type SystemData = (
                WriteStorage<'a, $graphcomp>,
                ReadStorage<'a, PhysicsBody>,
                Option<Read<'a, BodySet>>,
            );

            // TODO add a limit to length of graph
            fn run(&mut self, (mut speed_graphs, physics_bodies, body_set): Self::SystemData) {
                (&mut speed_graphs, &physics_bodies)
                    .join()
                    .for_each(|(graph, physics_body)| {
                        let rigid_body = body_set
                            .as_ref()
                            .unwrap()
                            .get(physics_body.body_handle)
                            .unwrap()
                            .downcast_ref::<RigidBody>()
                            .unwrap();
                        // let speed = rigid_body.velocity().linear.norm();
                        let val = $access_fn(rigid_body);
                        graph.add_val(val);
                    });
            }
        }
    };
}

make_graphsys!(SpeedGraphSys, SpeedGraph, |rigid_body: &RigidBody| {
    rigid_body.velocity().linear.norm()
});
make_graphsys!(RotVelGraphSys, RotVelGraph, |rigid_body: &RigidBody| {
    rigid_body.velocity().angular
});
