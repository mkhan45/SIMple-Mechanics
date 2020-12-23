use specs::prelude::*;

use crate::{GeometricalWorld, BodySet, ColliderSet, JointConstraintSet, ForceGeneratorSet, MechanicalWorld};
use crate::resources::{Paused, FrameSteps, Timestep};

pub struct PhysicsSys;

impl<'a> System<'a> for PhysicsSys {
    #[allow(clippy::clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, GeometricalWorld>,
        WriteExpect<'a, BodySet>,
        WriteExpect<'a, ColliderSet>,
        WriteExpect<'a, JointConstraintSet>,
        WriteExpect<'a, ForceGeneratorSet>,
        WriteExpect<'a, MechanicalWorld>,
        Read<'a, Timestep>,
        Read<'a, Paused>,
        Read<'a, FrameSteps>,
    );

    fn run(
        &mut self,
        (
            mut geometrical_world,
            mut body_set,
            mut collider_set,
            mut joint_constraint_set,
            mut force_generator_set,
            mut mechanical_world,
            timestep,
            paused,
            frame_steps,
        ): Self::SystemData,
    ) {
        // not running the physics step at all when paused causes some weird behavior,
        // so just run it with a timestep of 0
        if paused.0 {
            mechanical_world.set_timestep(0.0);
        } else {
            mechanical_world.set_timestep(timestep.0);
        }

        (0..frame_steps.0).for_each(|_| {
            mechanical_world.step(
                &mut *geometrical_world,
                &mut *body_set,
                &mut *collider_set,
                &mut *joint_constraint_set,
                &mut *force_generator_set,
            );
        });
    }
}
