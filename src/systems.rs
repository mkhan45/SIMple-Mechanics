#![allow(clippy::type_complexity)]
use specs::prelude::*;

use crate::{BodySet, MechanicalWorld, RigidBody, Selected};

use crate::components::*;
use crate::gui::graphs::{LineGraph, SpeedGraph};

use crate::resources::*;

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
    type SystemData = (ReadStorage<'a, SpeedGraph>, Write<'a, GraphMinMax>);

    fn run(&mut self, (speed_graphs, mut min_max): Self::SystemData) {
        let (mut min, mut max) = (std::f32::INFINITY, std::f32::NEG_INFINITY);

        speed_graphs.join().for_each(|graph| {
            let (s0, s1) = graph.points();
            s0.iter().chain(s1.iter()).for_each(|[_, v]| {
                min = min.min(*v);
                max = max.max(*v);
            });
        });

        min_max.0 = min;
        min_max.1 = max;
    }
}

pub struct SpeedGraphSys;
impl<'a> System<'a> for SpeedGraphSys {
    type SystemData = (
        WriteStorage<'a, SpeedGraph>,
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
                let speed = rigid_body.velocity().linear.norm();
                graph.add_val(speed);
            });
    }
}
