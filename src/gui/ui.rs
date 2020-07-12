use imgui::*;

use crate::gui::imgui_wrapper::*;
use crate::{
    components::{Collider, PhysicsBody},
    resources::{
        CreateElasticity, CreateFriction, CreateMass, CreateShapeCentered, CreateShapeStatic,
        Resolution, ShapeInfo,
    },
    BodySet, ColliderSet, RigidBody,
};

use nphysics2d::material::BasicMaterial;
use specs::prelude::*;

macro_rules! signal_button {
    ( $label:expr, $signal:expr, $ui:expr, $signals:expr) => {
        if $ui.small_button(im_str!($label)) {
            $signals.push($signal);
        }
    };
}

pub fn make_menu_bar(ui: &mut imgui::Ui, signals: &mut Vec<UiSignal>, world: &mut World) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("Create Shape"), true, || {
            ui.drag_float(im_str!("Mass"), &mut world.fetch_mut::<CreateMass>().0)
                .min(0.001)
                .max(250.0)
                .speed(0.25)
                .build();

            ui.drag_float(
                im_str!("Elasticity"),
                &mut world.fetch_mut::<CreateElasticity>().0,
            )
            .min(0.00)
            .max(1.0)
            .speed(0.1)
            .build();

            ui.drag_float(
                im_str!("Friction"),
                &mut world.fetch_mut::<CreateFriction>().0,
            )
            .min(0.00)
            .max(1.0)
            .speed(0.1)
            .build();

            ui.checkbox(
                im_str!("Centered"),
                &mut world.get_mut::<CreateShapeCentered>().unwrap().0,
            );

            ui.checkbox(
                im_str!("Static"),
                &mut world.get_mut::<CreateShapeStatic>().unwrap().0,
            );

            signal_button!(
                "Rectangle",
                UiSignal::AddShape(ShapeInfo::Rectangle(None)),
                ui,
                signals
            );
            signal_button!(
                "Circle",
                UiSignal::AddShape(ShapeInfo::Circle(None)),
                ui,
                signals
            );
            signal_button!(
                "Polygon",
                UiSignal::AddShape(ShapeInfo::Polygon(Some(Vec::with_capacity(3)))),
                ui,
                signals
            );
        });
    });
}

pub fn make_sidemenu(
    ui: &mut imgui::Ui,
    world: &World,
    entity: Entity,
    signals: &mut Vec<UiSignal>,
) {
    let mut body_set = world.fetch_mut::<BodySet>();
    let physics_body = {
        let physics_bodies = world.read_storage::<PhysicsBody>();
        let physics_body_handle = physics_bodies.get(entity).unwrap();
        body_set
            .get_mut(physics_body_handle.body_handle)
            .unwrap()
            .downcast_mut::<RigidBody>()
            .unwrap()
    };

    let mut collider_set = world.fetch_mut::<ColliderSet>();
    let body_collider = {
        let colliders = world.read_storage::<Collider>();
        let collider_handle = colliders.get(entity).unwrap();
        collider_set.get_mut(collider_handle.coll_handle).unwrap()
    };

    let resolution = world.fetch::<Resolution>().0;
    let win = imgui::Window::new(im_str!("Object Info"))
        .position([0.0, 30.0], imgui::Condition::Always)
        .size(
            [resolution.x * 0.40, resolution.y - 30.0],
            imgui::Condition::Appearing,
        )
        .size_constraints(
            [resolution.x * 0.2, resolution.y - 30.0],
            [resolution.x * 0.6, resolution.y - 30.0],
        )
        .collapsible(false)
        .movable(false);

    win.build(ui, || {
        let pos = physics_body.position();
        let vel = physics_body.velocity();

        ui.text(im_str!("\nObject Info"));
        ui.text(format!(
            "Position: {:.2}, {:.2}",
            pos.translation.x, pos.translation.y
        ));
        ui.text(format!("Rotation: {:.2}", pos.rotation.angle()));
        ui.text(format!(
            "Velocity: {:.2}, {:.2}",
            vel.linear.x, vel.linear.y
        ));
        ui.text(format!("Angular Velocity: {:.2}", vel.angular));

        let mut mass = physics_body.augmented_mass().linear;
        ui.drag_float(im_str!("Mass"), &mut mass)
            .min(0.0)
            .max(250.0)
            .speed(0.25)
            .build();
        physics_body.set_mass(mass);

        let material = body_collider.material_mut();
        let basic_material = material.downcast_mut::<BasicMaterial<f32>>().unwrap();
        ui.drag_float(im_str!("Friction"), &mut basic_material.friction)
            .min(0.0)
            .max(1.0)
            .speed(0.1)
            .build();

        ui.drag_float(im_str!("Elasticity"), &mut basic_material.restitution)
            .min(0.0)
            .max(1.0)
            .speed(0.1)
            .build();

        signal_button!("Delete Shape", UiSignal::DeleteShape(entity), ui, signals);
    });
}

pub fn make_default_ui(ui: &mut imgui::Ui) {
    // Window
    imgui::Window::new(im_str!("Hello world"))
        .position([100.0, 100.0], imgui::Condition::Appearing)
        .build(ui, || {
            ui.text(im_str!("Hello world!"));
            ui.separator();

            if ui.small_button(im_str!("small button")) {
                println!("Small button clicked");
            }
        });
}
