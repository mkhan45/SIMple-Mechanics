use specs::prelude::*;

use crate::gui::graphs::{
    LineGraph, RotGraph, RotVelGraph, SpeedGraph, XPosGraph, XVelGraph, YPosGraph, YVelGraph,
};

use crate::components::PhysicsBody;
use crate::resources::Paused;
use crate::{BodySet, RigidBody};

use microprofile::scope;

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
        microprofile::scope!("graphs", "line_graph");
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
