use specs::prelude::*;

pub struct SelectedMoveSys;

use crate::{BodySet, RigidBody, Selected, MechanicalWorld};

use crate::components::*;

use crate::resources::*;

impl<'a> System<'a> for SelectedMoveSys {
    type SystemData = (
        ReadStorage<'a, Selected>,
        ReadStorage<'a, PhysicsBody>,
        Read<'a, MousePos>,
        Read<'a, DT>,
        Option<Read<'a, MechanicalWorld>>,
        Option<Write<'a, BodySet>>,
    );

    fn run(&mut self, (selected, physics_body, mouse_pos, dt, mechanical_world, mut body_set): Self::SystemData) {
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
