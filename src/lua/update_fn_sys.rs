use rlua::prelude::*;
use specs::prelude::*;

use crate::components::{Color, Name, PhysicsBody, UpdateFunction};
use crate::resources::{LuaRes, Paused};
use crate::{BodySet, RigidBody, Vector};

use microprofile::scope;

use nphysics2d as np;

pub struct LuaUpdateFnSys;

impl<'a> System<'a> for LuaUpdateFnSys {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, BodySet>,
        ReadStorage<'a, UpdateFunction>,
        ReadStorage<'a, PhysicsBody>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Color>,
        Read<'a, LuaRes>,
        Read<'a, Paused>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            mut body_set,
            update_functions,
            physics_bodies,
            names,
            mut colors,
            lua_res,
            paused,
            entities,
        ): Self::SystemData,
    ) {
        if !paused.0 {
            lua_res.lock().unwrap().context(|lua_ctx| {
                let globals = lua_ctx.globals();

                (&update_functions, &physics_bodies, &mut colors, &entities)
                    .join()
                    .for_each(|(UpdateFunction(fn_name), physics_body, color, entity)| {
                        let update_function: LuaFunction = globals.get(fn_name.as_str()).unwrap();
                        let rigid_body = &mut body_set
                            .get_mut(physics_body.body_handle)
                            .unwrap()
                            .downcast_mut::<RigidBody>()
                            .unwrap();

                        let obj_table = table_from_rigid_body(&rigid_body, &lua_ctx);
                        let color_table = table_from_color(&color, &lua_ctx);
                        obj_table.set("color", color_table).unwrap();

                        // you can't change the name in the Lua but they're
                        // useful to read
                        if let Some(Name(obj_name)) = names.get(entity) {
                            obj_table.set("name", obj_name.clone()).unwrap();
                        }

                        if let Ok(changed_obj_table) =
                            update_function.call::<_, LuaTable>(obj_table)
                        {
                            update_rigid_body_from_table(rigid_body, &changed_obj_table);

                            let change_color_table: LuaTable =
                                changed_obj_table.get("color").unwrap();
                            update_color_from_table(&mut *color, &change_color_table);
                        }
                    });
            });
        }
    }
}

fn table_from_color<'a>(color: &Color, lua_ctx: &LuaContext<'a>) -> LuaTable<'a> {
    microprofile::scope!("lua", "Lua serialize color table");
    let r = color.0.r * 255.0;
    let g = color.0.g * 255.0;
    let b = color.0.b * 255.0;

    let c_table = lua_ctx.create_table().unwrap();
    c_table.set("r", r).unwrap();
    c_table.set("g", g).unwrap();
    c_table.set("b", b).unwrap();

    c_table
}

fn update_color_from_table<'a>(color: &mut Color, table: &LuaTable<'a>) {
    microprofile::scope!("lua", "Deserialize color table from Lua");
    color.0.r = table.get::<_, f32>("r").unwrap() / 255.0;
    color.0.g = table.get::<_, f32>("g").unwrap() / 255.0;
    color.0.b = table.get::<_, f32>("b").unwrap() / 255.0;
}

fn table_from_rigid_body<'a>(rigid_body: &RigidBody, lua_ctx: &LuaContext<'a>) -> LuaTable<'a> {
    microprofile::scope!("lua", "Lua serialize body table");
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
    obj_table.set("y_vel", vel.y).unwrap();

    obj_table
}

fn update_rigid_body_from_table<'a>(rigid_body: &mut RigidBody, table: &LuaTable<'a>) {
    microprofile::scope!("lua", "Deserialize rigid body table from Lua");
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
