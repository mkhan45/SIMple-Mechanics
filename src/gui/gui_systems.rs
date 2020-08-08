#![allow(clippy::type_complexity)]
use specs::prelude::*;

use crate::{BodySet, ColliderSet, MechanicalWorld, RigidBody, Selected};

use crate::components::*;
use crate::gui::graphs::{
    LineGraph, RotGraph, RotVelGraph, SpeedGraph, XPosGraph, XVelGraph, YPosGraph, YVelGraph,
};

use crate::{resources::*, Vector};

pub struct SelectedMoveSys;
impl<'a> System<'a> for SelectedMoveSys {
    type SystemData = (
        ReadStorage<'a, Selected>,
        ReadStorage<'a, PhysicsBody>,
        ReadStorage<'a, Collider>,
        Read<'a, MousePos>,
        Read<'a, Paused>,
        Option<Write<'a, ColliderSet>>,
        Option<Read<'a, MechanicalWorld>>,
        Option<Write<'a, BodySet>>,
    );

    fn run(
        &mut self,
        (
            selected,
            physics_bodies,
            colliders,
            mouse_pos,
            paused,
            collider_set,
            mechanical_world,
            mut body_set,
        ): Self::SystemData,
    ) {
        let body_set = body_set.as_mut().unwrap();
        let mut collider_set = collider_set.unwrap();

        // If not paused ,change velocity. If paused, change position directly and set velocity to
        // 0
        if !paused.0 {
            (&selected, &physics_bodies)
                .join()
                .for_each(|(_, physics_body)| {
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
        } else {
            (&selected, &physics_bodies, &colliders).join().for_each(
                |(_, physics_body, collider_handle)| {
                    let rigid_body = body_set
                        .get_mut(physics_body.body_handle)
                        .unwrap()
                        .downcast_mut::<RigidBody>()
                        .unwrap();
                    let collider = collider_set.get_mut(collider_handle.coll_handle);

                    let mut rigid_body_isometry = *rigid_body.position();
                    rigid_body_isometry.translation.vector = mouse_pos.0;
                    rigid_body.set_position(rigid_body_isometry);

                    collider.unwrap().set_position(rigid_body_isometry);
                    rigid_body.set_linear_velocity(Vector::new(0.0, 0.0));
                },
            );
        }
    }
}

pub struct MinMaxGraphSys;
impl<'a> System<'a> for MinMaxGraphSys {
    type SystemData = (
        ReadStorage<'a, SpeedGraph>,
        ReadStorage<'a, RotVelGraph>,
        ReadStorage<'a, XVelGraph>,
        ReadStorage<'a, YVelGraph>,
        ReadStorage<'a, XPosGraph>,
        ReadStorage<'a, YPosGraph>,
        ReadStorage<'a, RotGraph>,
        Read<'a, Paused>,
        Write<'a, GraphMinMax>,
    );

    fn run(
        &mut self,
        (
            speed_graphs,
            rotvel_graphs,
            xvel_graphs,
            yvel_graphs,
            xpos_graphs,
            ypos_graphs,
            rot_graphs,
            paused,
            mut min_max,
        ): Self::SystemData,
    ) {
        if paused.0 {
            return;
        }

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
        minmax_graph_storage!(xvel_graphs);
        minmax_graph_storage!(yvel_graphs);
        minmax_graph_storage!(xpos_graphs);
        minmax_graph_storage!(ypos_graphs);
        minmax_graph_storage!(rot_graphs);

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

#[derive(Default)]
pub struct LineGraphSys<T>
where
    T: LineGraph + Component,
{
    _phantom_data: std::marker::PhantomData<T>,
}

impl<'a, T> System<'a> for LineGraphSys<T>
where
    T: LineGraph + Component,
{
    type SystemData = (
        WriteStorage<'a, T>,
        ReadStorage<'a, PhysicsBody>,
        Read<'a, Paused>,
        Option<Read<'a, BodySet>>,
    );

    fn run(&mut self, (mut graphs, physics_bodies, paused, body_set): Self::SystemData) {
        if paused.0 {
            return;
        }

        (&mut graphs, &physics_bodies)
            .join()
            .for_each(|(graph, physics_body)| {
                let rigid_body = body_set
                    .as_ref()
                    .unwrap()
                    .get(physics_body.body_handle)
                    .unwrap()
                    .downcast_ref::<RigidBody>()
                    .unwrap();
                let val = T::access_field(rigid_body);
                graph.add_val(val);
            });
    }
}

pub type SpeedGraphSys = LineGraphSys<SpeedGraph>;
pub type RotVelGraphSys = LineGraphSys<RotVelGraph>;
pub type XVelGraphSys = LineGraphSys<XVelGraph>;
pub type YVelGraphSys = LineGraphSys<YVelGraph>;
pub type XPosGraphSys = LineGraphSys<XPosGraph>;
pub type YPosGraphSys = LineGraphSys<YPosGraph>;
pub type RotGraphSys = LineGraphSys<RotGraph>;
