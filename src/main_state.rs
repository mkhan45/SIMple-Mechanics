use bevy_ecs::prelude::*;

use crate::{physics::{PhysicsRes, physics_step_sys}, main};

pub struct MainState {
    pub world: World,
    physics_schedule: Schedule,
}

impl Default for MainState {
    fn default() -> Self {
        let world = {
            let mut world = World::new();
            world.insert_resource(PhysicsRes::default());

            world
        };

        let physics_schedule = {
            let mut main_physics_schedule = Schedule::default();

            main_physics_schedule.add_stage("physics", SystemStage::single(physics_step_sys.system()));

            main_physics_schedule
        };

        MainState { world, physics_schedule, }
    }
}
