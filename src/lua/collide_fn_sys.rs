use rlua::prelude::*;
use specs::prelude::*;

use crate::components::{CollideFunction, Color, Name, PhysicsBody};
use crate::resources::{LuaRes, Paused};
use crate::{BodySet, ColliderSet, GeometricalWorld, RigidBody, Vector};

use nphysics2d as np;

pub struct LuaCollideFnSys;

impl<'a> System<'a> for LuaCollideFnSys {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, BodySet>,
        ReadExpect<'a, GeometricalWorld>,
        ReadExpect<'a, ColliderSet>,
        ReadStorage<'a, CollideFunction>,
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
            geometrical_world,
            collider_set,
            collide_functions,
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
                (&collide_functions, &physics_bodies, &mut colors, &entities)
                    .join()
                    .for_each(|(CollideFunction(fn_name), physics_body, color, entity)| {
                        let collide_function: LuaFunction = globals.get(fn_name.as_str()).unwrap();

                        if let Some(contacts) = geometrical_world.contacts_with(
                            &*collider_set,
                            physics_body.body_handle,
                            true,
                        ) {
                            contacts.for_each(|(handle_1, collider_1, handle_2, collider_2, _algo, manifold)|{
                            });
                        }
                    });
            });
        }
    }
}
