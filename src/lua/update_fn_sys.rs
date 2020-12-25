use rlua::prelude::*;
use specs::prelude::*;

use crate::components::{PhysicsBody, UpdateFunction};
use crate::resources::LuaRes;
use crate::{BodySet, RigidBody, Vector};

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

            (&update_functions, &physics_bodies).join().for_each(
                |(UpdateFunction(fn_name), physics_body)| {
                    let update_function: LuaFunction = globals.get(fn_name.as_str()).unwrap();
                    let rigid_body = &mut body_set
                        .get_mut(physics_body.body_handle)
                        .unwrap()
                        .downcast_mut::<RigidBody>()
                        .unwrap();

                    let obj_table = table_from_rigid_body(&rigid_body, &lua_ctx);

                    if let Ok(changed_obj_table) = update_function.call::<_, LuaTable>(obj_table) {
                        update_rigid_body_from_table(rigid_body, &changed_obj_table);
                    }
                },
            );
        });
    }
}

fn table_from_rigid_body<'a>(rigid_body: &RigidBody, lua_ctx: &LuaContext<'a>) -> LuaTable<'a> {
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

    obj_table
}

fn update_rigid_body_from_table<'a>(rigid_body: &mut RigidBody, table: &LuaTable<'a>) {
    let new_pos = {
        let new_x: f32 = table.get("x").unwrap();
        let new_y: f32 = table.get("y").unwrap();
        let new_rot: f32 = table.get("rot").unwrap();
        np::math::Isometry::new(Vector::new(new_x, new_y), new_rot)
    };

    let new_vel = {
        let new_x_vel: f32 = table.get("x_vel").unwrap();
        let new_y_vel: f32 = table.get("y_vel").unwrap();
        Vector::new(new_x_vel, new_y_vel)
    };

    rigid_body.set_position(new_pos);
    rigid_body.set_linear_velocity(new_vel);
}
