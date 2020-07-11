use imgui::*;

use crate::gui::imgui_wrapper::*;
use crate::{
    components::PhysicsBody,
    resources::{
        CreateElasticity, CreateFriction, CreateMass, CreateShapeCentered, Resolution, ShapeInfo,
        SideMenuShown,
    },
    BodySet, RigidBody,
};

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
                im_str!("Center Shapes"),
                &mut world.get_mut::<CreateShapeCentered>().unwrap().0,
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
    let body_set = world.fetch::<BodySet>();
    let physics_body = {
        let physics_bodies = world.read_storage::<PhysicsBody>();
        let physics_body_handle = physics_bodies.get(entity).unwrap();
        body_set
            .get(physics_body_handle.body_handle)
            .unwrap()
            .downcast_ref::<RigidBody>()
            .unwrap()
    };

    let pos = physics_body.position();
    let vel = physics_body.velocity();

    let resolution = world.fetch::<Resolution>().0;
    let mut sidemenu_shown = world.fetch_mut::<SideMenuShown>().0;
    let win = imgui::Window::new(im_str!("Object Info"))
        .position([0.0, 30.0], imgui::Condition::Always)
        .opened(&mut sidemenu_shown)
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
