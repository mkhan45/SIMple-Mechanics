use specs::prelude::*;

use crate::resources::{MousePos, Paused, Selected};
use crate::{BodySet, Collider, ColliderSet, MechanicalWorld, PhysicsBody, RigidBody, Vector};

pub struct SelectedMoveSys;
impl<'a> System<'a> for SelectedMoveSys {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'a, Selected>,
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
        if let Some(selected) = selected.0 {
            let physics_body = physics_bodies.get(selected).unwrap();
            let rigid_body = body_set
                .get_mut(physics_body.body_handle)
                .unwrap()
                .downcast_mut::<RigidBody>()
                .unwrap();

            if !paused.0 {
                let pos = rigid_body.position().translation.vector;
                let new_vel = mouse_pos.0 - pos;
                let physics_step = mechanical_world.as_ref().unwrap().timestep();
                rigid_body.set_linear_velocity(new_vel / physics_step);
            } else {
                let collider_handle = colliders.get(selected).unwrap();
                let collider = collider_set.get_mut(collider_handle.coll_handle);

                let mut rigid_body_isometry = *rigid_body.position();
                rigid_body_isometry.translation.vector = mouse_pos.0;
                rigid_body.set_position(rigid_body_isometry);

                collider.unwrap().set_position(rigid_body_isometry);
                rigid_body.set_linear_velocity(Vector::new(0.0, 0.0));
            }
        }
    }
}
