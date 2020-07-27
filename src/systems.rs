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
