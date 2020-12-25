use specs::prelude::*;
use rlua::prelude::*;

use crate::{BodySet, RigidBody, Vector};
use crate::components::{UpdateFunction, PhysicsBody};
use crate::resources::LuaRes;

use nphysics2d as np;

pub struct LuaUpdateFnSys;

impl<'a> System<'a> for LuaUpdateFnSys {
    type SystemData = (
        WriteExpect<'a, BodySet>,
        ReadStorage<'a, UpdateFunction>,
        ReadStorage<'a, PhysicsBody>,
        Read<'a, LuaRes>,
    );

    fn run(&mut self, (mut body_set, update_functions, physics_bodies, lua_res): Self::SystemData) {
        lua_res.lock().unwrap().context(|lua_ctx| {
            let globals = lua_ctx.globals();

            (&update_functions, &physics_bodies).join().for_each(|(UpdateFunction(fn_name), physics_body)| {
                let update_function: LuaFunction = globals.get(fn_name.as_str()).unwrap();
                let rigid_body = body_set.get_mut(physics_body.body_handle).unwrap().downcast_mut::<RigidBody>().unwrap();

                let (pos, rot) = {
                    let isometry = rigid_body.position();
                    (isometry.translation, isometry.rotation.angle())
                };
                let vel: Vector = rigid_body.velocity().linear;

                let obj_table = lua_ctx.create_table().unwrap();
                obj_table.set("x", pos.x).unwrap();
                obj_table.set("y", pos.y).unwrap();
                obj_table.set("rot", rot).unwrap();
                obj_table.set("x_vel", vel.x).unwrap();
                obj_table.set("y_vel", vel.y).unwrap();

                let changed_obj = update_function.call::<_, LuaTable>(obj_table).expect("error running lua fn");

                let new_pos = {
                    let new_x: f32 = changed_obj.get("x").unwrap();
                    let new_y: f32 = changed_obj.get("y").unwrap();
                    let new_rot: f32 = changed_obj.get("rot").unwrap();
                    np::math::Isometry::new(Vector::new(new_x, new_y), new_rot)
                };

                let new_vel = {
                    let new_x_vel: f32 = changed_obj.get("x_vel").unwrap();
                    let new_y_vel: f32 = changed_obj.get("y_vel").unwrap();
                    Vector::new(new_x_vel, new_y_vel)
                };

                rigid_body.set_position(new_pos);
                rigid_body.set_linear_velocity(new_vel);
            });
        });
    }
}
