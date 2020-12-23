use ggez::input::keyboard::KeyCode;

use specs::prelude::*;

use crate::{
    BodySet, Collider, ColliderSet, Vector,
};

use crate::components::*;

use crate::resources::Camera;

use crate::gui::imgui_wrapper::{ImGuiWrapper, UiChoice};

pub mod body_builder;
mod util;

mod draw_shape_sys;

mod event_handler;
pub use event_handler::*;

pub mod physics_sys;

pub struct MainState<'a, 'b> {
    pub world: specs::World,
    pub dispatcher: Dispatcher<'a, 'b>,
    pub imgui_wrapper: ImGuiWrapper,
}

impl<'a, 'b> MainState<'a, 'b> {
    pub fn delete_entity(&mut self, entity: Entity) {
        // to delete an entity, it needs to be removed
        // from the nphysics body and collider sets
        // before being removed from the specs world.
        // NEVER call world.delete_entity() to remove
        // a physics object.
        {
            let mut body_set = self.world.fetch_mut::<BodySet>();
            let body_storage = self.world.read_storage::<PhysicsBody>();
            let body_handle = body_storage.get(entity).unwrap();

            let mut collider_set = self.world.fetch_mut::<ColliderSet>();
            let collider_storage = self.world.read_storage::<Collider>();
            let collider_handle = collider_storage.get(entity).unwrap();

            body_set.remove(body_handle.body_handle);
            collider_set.remove(collider_handle.coll_handle);
        }

        self.imgui_wrapper.remove_sidemenu();
        self.world.delete_entity(entity).unwrap();
    }

    pub fn delete_all(&mut self) {
        let delete_buff: Vec<Entity> = {
            let physics_bodies = self.world.read_storage::<PhysicsBody>();
            let entities = self.world.entities();
            (&physics_bodies, &entities)
                .join()
                .map(|(_, e)| e)
                .collect()
        };

        delete_buff.iter().for_each(|entity| {
            self.delete_entity(*entity);
        });
    }

    pub fn reactivate_all(&mut self) {
        let bodies = self.world.read_storage::<PhysicsBody>();
        let mut body_set = self.world.fetch_mut::<BodySet>();

        bodies.join().for_each(|body| {
            body_set.get_mut(body.body_handle).unwrap().activate();
        });
    }

    pub fn update_sidemenu(&mut self) {
        // only one physics body should have the InfoDisplayed component;
        // maybe it should be a resource: TODO
        let info_displayed = self.world.read_storage::<InfoDisplayed>();
        let entities = self.world.entities();
        if let Some((_, entity)) = (&info_displayed, &entities).join().next() {
            self.imgui_wrapper.remove_sidemenu();
            self.imgui_wrapper
                .shown_menus
                .insert(UiChoice::SideMenu(entity));
        }
    }

    pub fn move_camera(&mut self, ctx: &mut ggez::Context) {
        use ggez::input::keyboard;

        let mut camera = self.world.fetch_mut::<Camera>();
        const SPEED: f32 = 0.5;

        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            camera.translate(Vector::new(0.0, -SPEED));
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            camera.translate(Vector::new(0.0, SPEED));
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            camera.translate(Vector::new(-SPEED, 0.0));
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            camera.translate(Vector::new(SPEED, 0.0));
        }
    }
}
