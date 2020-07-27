use imgui::*;

use std::convert::TryInto;

use nphysics2d as np;

use crate::gui::imgui_wrapper::*;
use crate::{
    components::{Collider, Color, Name, PhysicsBody},
    resources::{
        CreateElasticity, CreateFriction, CreateMass, CreateShapeCentered, CreateShapeStatic,
        FrameSteps, Resolution, ShapeInfo,
    },
    BodySet, ColliderSet, MechanicalWorld, RigidBody, Vector,
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
            .speed(0.05)
            .build();

            ui.drag_float(
                im_str!("Friction"),
                &mut world.fetch_mut::<CreateFriction>().0,
            )
            .min(0.00)
            .max(1.0)
            .speed(0.05)
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
            signal_button!(
                "Line",
                UiSignal::AddShape(ShapeInfo::Polyline(Some(Vec::with_capacity(30)))),
                ui,
                signals
            );
        });

        ui.menu(im_str!("Global Variables"), true, || {
            let mut mechanical_world = world.fetch_mut::<MechanicalWorld>();

            let mut timestep = mechanical_world.timestep();
            ui.drag_float(im_str!("Timestep"), &mut timestep)
                .min(1e-10)
                .max(2.0)
                .speed(0.01)
                .build();
            mechanical_world.set_timestep(timestep);

            ui.drag_float(im_str!("Gravity"), &mut mechanical_world.gravity.y)
                .speed(0.1)
                .build();

            {
                std::mem::drop(mechanical_world);
                let mut frame_steps_i32 = world.fetch_mut::<FrameSteps>().0 as i32;
                ui.drag_int(im_str!("Steps Per Frame"), &mut frame_steps_i32)
                    .min(1)
                    .max(250)
                    .build();
                world.insert(FrameSteps(frame_steps_i32.try_into().unwrap()));
            }
        });

        signal_button!("Clear Scene", UiSignal::DeleteAll, ui, signals);
        signal_button!("Pause", UiSignal::TogglePause, ui, signals);

        ui.menu(im_str!("Load"), true, || {
            let dir = std::path::Path::new("./lua");
            match std::fs::read_dir(dir) {
                Ok(dir_entries) => {
                    dir_entries.for_each(|entry| {
                        if let Ok(entry) = entry {
                            let filename = entry.file_name().to_string_lossy().into_owned();
                            let mut utf8_filename = String::new();
                            filename.chars().for_each(|c| utf8_filename.push(c));
                            let imstring_filename = ImString::new(utf8_filename);

                            if &filename.as_str()[filename.len() - 4..] == ".lua" {
                                let label = imstring_filename;
                                if ui.small_button(&label) {
                                    signals.push(UiSignal::LoadLua(filename));
                                }
                            }
                        }
                    });
                }
                Err(e) => println!("Error reading dir: {}", e),
            }
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
    let mut names = world.write_storage::<Name>();

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
        ui.text(im_str!("\nObject Info"));

        let mut name_buff = match names.get(entity) {
            Some(name) => ImString::new(name.0.clone()),
            None => ImString::new(""),
        };
        ui.input_text(im_str!("Name"), &mut name_buff).build();
        if name_buff.to_str() != "" {
            let name = name_buff.to_string();
            names.insert(entity, Name(name)).unwrap();
        }

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
            .speed(0.05)
            .build();

        ui.drag_float(im_str!("Elasticity"), &mut basic_material.restitution)
            .min(0.0)
            .max(1.0)
            .speed(0.05)
            .build();

        let pos = physics_body.position();
        let mut linear_pos = [pos.translation.x, pos.translation.y];
        ui.drag_float2(im_str!("Position"), &mut linear_pos)
            .speed(0.05)
            .build();
        let mut angular_pos = pos.rotation.angle();
        ui.drag_float(im_str!("Rotation"), &mut angular_pos)
            .speed(0.05)
            .build();

        let translation = Vector::new(linear_pos[0], linear_pos[1]);
        physics_body.set_position(np::math::Isometry::new(translation, angular_pos));

        let vel = physics_body.velocity();
        let mut linear_vel = [vel.linear.x, vel.linear.y];
        let mut angular_vel = vel.angular;
        ui.drag_float2(im_str!("Velocity"), &mut linear_vel)
            .speed(0.05)
            .build();
        ui.drag_float(im_str!("Angular Velocity"), &mut angular_vel)
            .speed(0.05)
            .build();
        physics_body.set_linear_velocity(Vector::new(linear_vel[0], linear_vel[1]));
        physics_body.set_angular_velocity(angular_vel);

        let mut colors_storage = world.write_storage::<Color>();
        let mut color_arr = {
            let color = colors_storage.get(entity).unwrap().0;
            [color.r, color.g, color.b]
        };
        ui.drag_float3(im_str!("RGB"), &mut color_arr)
            .min(0.0)
            .max(1.0)
            .speed(0.01)
            .build();
        let mut color = colors_storage.get_mut(entity).unwrap();
        color.0.r = color_arr[0];
        color.0.g = color_arr[1];
        color.0.b = color_arr[2];

        signal_button!("Graph Speed", UiSignal::AddSpeedGraph(entity), ui, signals);
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
